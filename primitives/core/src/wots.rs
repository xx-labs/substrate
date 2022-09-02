// Copyright 2022 ChainSafe Systems and xx labs
// SPDX-License-Identifier: LGPL-3.0-only

// tag::description[]
//! Simple W-OTS+ API.
// end::description[]
#[cfg(feature = "std")]
use crate::{crypto::Ss58Codec};
#[cfg(feature = "std")]
use bip39::{Language, Mnemonic, MnemonicType};
use codec::MaxEncodedLen;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    crypto::{
        ByteArray, CryptoType, CryptoTypeId, CryptoTypePublicPair, Public as TraitPublic,
        Deref, Derive, UncheckedFrom,
    },
    Decode, Encode, H256,
};
#[cfg(feature = "full_crypto")]
use crate::{
    crypto::{DeriveJunction, SecretStringError},
    Pair as TraitPair,
};
use sp_runtime_interface::pass_by::PassByInner;
use sp_std::vec::Vec;
#[cfg(feature = "full_crypto")]
use w_ots::{hasher::{Blake2bHasher, Sha3_256Hasher}, keys::Key, security::{consensus_params, verify as wots_verify}};

/// An identifier used to match public keys against wots+ keys
pub const CRYPTO_ID: CryptoTypeId = CryptoTypeId(*b"wots");

/// W-OTS Seed is 64 bytes, split into 2 parts of 32 bytes -> [secret seed | public seed]
#[cfg(feature = "full_crypto")]
#[derive(Clone, Encode, Decode)]
pub struct Seed(pub [u8; 64]);

#[cfg(feature = "full_crypto")]
impl Default for Seed {
    fn default() -> Self {
        Self([0; 64])
    }
}

#[cfg(feature = "full_crypto")]
impl AsRef<[u8]> for Seed {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "full_crypto")]
impl AsMut<[u8]> for Seed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[cfg(feature = "full_crypto")]
impl From<[u8; 64]> for Seed {
    fn from(s: [u8; 64]) -> Self {
        Self(s)
    }
}

/// A W-OTS+ public key.
#[cfg_attr(feature = "full_crypto", derive(Hash))]
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Encode,
    Decode,
    PassByInner,
    MaxEncodedLen,
    TypeInfo,
)]
pub struct Public(pub [u8; 32]);

/// A W-OTS+ key pair (with generic Hash functions)
#[cfg(feature = "full_crypto")]
pub struct Pair(pub Key<Blake2bHasher, Sha3_256Hasher>);

#[cfg(feature = "full_crypto")]
impl Clone for Pair {
    fn clone(&self) -> Self {
        Pair(self.0.clone())
    }
}

impl AsRef<[u8; 32]> for Public {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for Public {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Public {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl Deref for Public {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&[u8]> for Public {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != Self::LEN {
            return Err(());
        }
        let mut r = [0u8; Self::LEN];
        r.copy_from_slice(data);
        Ok(Self::unchecked_from(r))
    }
}

impl From<Public> for [u8; 32] {
    fn from(x: Public) -> Self {
        x.0
    }
}

#[cfg(feature = "full_crypto")]
impl From<Pair> for Public {
    fn from(x: Pair) -> Self {
        x.public()
    }
}

impl From<Public> for H256 {
    fn from(x: Public) -> Self {
        x.0.into()
    }
}

#[cfg(feature = "std")]
impl std::str::FromStr for Public {
    type Err = crate::crypto::PublicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ss58check(s)
    }
}

impl UncheckedFrom<[u8; 32]> for Public {
    fn unchecked_from(x: [u8; 32]) -> Self {
        Public::from_raw(x)
    }
}

impl UncheckedFrom<H256> for Public {
    fn unchecked_from(x: H256) -> Self {
        Public::from_h256(x)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Public {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_ss58check())
    }
}

impl core::fmt::Debug for Public {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = self.to_ss58check();
        write!(f, "{} ({}...)", crate::hexdisplay::HexDisplay::from(&self.0), &s[0..8])
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

#[cfg(feature = "std")]
impl Serialize for Public {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_ss58check())
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for Public {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Public::from_ss58check(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

/// A signature (a 512-bit value).
#[cfg_attr(feature = "full_crypto", derive(Hash))]
#[derive(Encode, Decode, TypeInfo, PassByInner, PartialEq, Eq)]
pub struct Signature(pub Vec<u8>);

impl MaxEncodedLen for Signature {
    fn max_encoded_len() -> usize {
        1121
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(Signature(data.to_vec()))
    }
}

#[cfg(feature = "std")]
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self))
    }
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let signature_hex = hex::decode(&String::deserialize(deserializer)?)
            .map_err(|e| de::Error::custom(format!("{:?}", e)))?;
        Signature::try_from(signature_hex.as_ref())
            .map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        Signature(self.0.clone())
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..]
    }
}

impl core::fmt::Debug for Signature {
    #[cfg(feature = "std")]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", crate::hexdisplay::HexDisplay::from(&self.0))
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut core::fmt::Formatter) -> core::fmt::Result {
        Ok(())
    }
}

impl Signature {
    /// A new instance from the given 64-byte `data`.
    ///
    /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
    /// you are certain that the array actually is a signature. GIGO!
    pub fn from_raw(data: &[u8]) -> Signature {
        Signature(data.to_vec())
    }

    /// A new instance from the given slice that should be 64 bytes long.
    ///
    /// NOTE: No checking goes on to ensure this is a real signature. Only use it if
    /// you are certain that the array actually is a signature. GIGO!
    pub fn from_slice(data: &[u8]) -> Option<Self> {
        Some(Self::from_raw(data))
    }
}

/// A localized signature also contains sender information.
#[cfg(feature = "std")]
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode)]
pub struct LocalizedSignature {
    /// The signer of the signature.
    pub signer: Public,
    /// The signature itself.
    pub signature: Signature,
}

impl Public {
    /// A new instance from the given 32-byte `data`.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_raw(data: [u8; 32]) -> Self {
        Public(data)
    }

    /// A new instance from an H256.
    ///
    /// NOTE: No checking goes on to ensure this is a real public key. Only use it if
    /// you are certain that the array actually is a pubkey. GIGO!
    pub fn from_h256(x: H256) -> Self {
        Public(x.into())
    }

    /// Return a slice filled with raw data.
    pub fn as_array_ref(&self) -> &[u8; 32] {
        self.as_ref()
    }
}

impl ByteArray for Public {
    const LEN: usize = 32;
}

impl TraitPublic for Public {
    fn to_public_crypto_pair(&self) -> CryptoTypePublicPair {
        CryptoTypePublicPair(CRYPTO_ID, self.to_raw_vec())
    }
}

impl Derive for Public {}

impl From<Public> for CryptoTypePublicPair {
    fn from(key: Public) -> Self {
        (&key).into()
    }
}

impl From<&Public> for CryptoTypePublicPair {
    fn from(key: &Public) -> Self {
        CryptoTypePublicPair(CRYPTO_ID, key.to_raw_vec())
    }
}

/// Derive a single hard junction.
#[allow(unused)]
#[cfg(feature = "full_crypto")]
pub fn derive_hard_junction(seed: &Seed, cc: &[u8; 32]) -> Seed {
    let sk_seed_bytes =
        ("WOTSHDKDSK", &seed.0[..32], cc).using_encoded(crate::hashing::blake2_256);
    let pk_seed_bytes =
        ("WOTSHDKDPK", &seed.0[32..], cc).using_encoded(crate::hashing::blake2_256);
    let seed_bytes = [sk_seed_bytes, pk_seed_bytes].concat();

    let mut seed = [0; 64];
    seed.copy_from_slice(&seed_bytes);

    Seed(seed)
}

/// An error when deriving a key.
#[cfg(feature = "full_crypto")]
pub enum DeriveError {
    /// A soft key was found in the path (and is unsupported).
    SoftKeyInPath,
    /// Invalid Seed,
    InvalidSeed,
}

#[cfg(feature = "full_crypto")]
impl TraitPair for Pair {
    type Public = Public;
    type Seed = Seed;
    type Signature = Signature;
    type DeriveError = DeriveError;

    /// Generate new secure (random) key pair and provide the recovery phrase.
    ///
    /// You can recover the same key later with `from_phrase`.
    #[cfg(feature = "std")]
    fn generate_with_phrase(password: Option<&str>) -> (Pair, String, Seed) {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.phrase();
        let (pair, seed) = Self::from_phrase(phrase, password)
            .expect("All phrases generated by Mnemonic are valid; qed");
        (pair, phrase.to_owned(), seed)
    }

    /// Generate key pair from given recovery phrase and password.
    #[cfg(feature = "std")]
    fn from_phrase(
        phrase: &str,
        password: Option<&str>,
    ) -> Result<(Pair, Seed), SecretStringError> {
        let big_seed = substrate_bip39::seed_from_entropy(
            Mnemonic::from_phrase(phrase, Language::English)
                .map_err(|_| SecretStringError::InvalidPhrase)?
                .entropy(),
            password.unwrap_or(""),
        )
        .map_err(|_| SecretStringError::InvalidSeed)?;

        if big_seed.len() < 64 {
            return Err(SecretStringError::InvalidSeedLength);
        }

        let mut bytes = [0; 64];
        bytes.copy_from_slice(&big_seed);
        let seed = Seed::from(bytes);

        Self::from_seed_slice(&bytes).map(|x| (x, seed))
    }

    /// Make a new key pair from secret seed material.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed(seed: &Seed) -> Pair {
        Self::from_seed_slice(seed.as_ref()).expect("seed has valid length; qed")
    }

    /// Make a new key pair from secret seed material. The slice must be 32 bytes long or it
    /// will return `None`.
    ///
    /// You should never need to use this; generate(), generate_with_phrase
    fn from_seed_slice(seed_slice: &[u8]) -> Result<Pair, SecretStringError> {
        let mut bytes = [0; 64];
        bytes.copy_from_slice(seed_slice);

        let seed = Seed::from(bytes);
        let (mut sk_seed, mut pk_seed) = ([0; 32], [0; 32]);
        sk_seed.copy_from_slice(&seed.0[..32]);
        pk_seed.copy_from_slice(&seed.0[32..]);

        Ok(Pair(
           Key::from_seed(consensus_params(), sk_seed, pk_seed)
                .map_err(|_| SecretStringError::InvalidFormat)?,
        ))
    }

    /// Derive a child key from a series of given junctions.
    fn derive<Iter: Iterator<Item = DeriveJunction>>(
        &self,
        path: Iter,
        _seed: Option<Seed>,
    ) -> Result<(Pair, Option<Seed>), DeriveError> {
        let mut acc = Seed::default();
        acc.0[..32].copy_from_slice(&self.0.seed);
        acc.0[32..].copy_from_slice(&self.0.p_seed);

        for j in path {
            match j {
                DeriveJunction::Soft(_) => return Err(DeriveError::SoftKeyInPath),
                DeriveJunction::Hard(cc) => acc = derive_hard_junction(&acc, &cc),
            }
        }
        Ok((Self::from_seed(&acc), Some(acc)))
    }

    /// Get the public key.
    fn public(&self) -> Public {
        let mut r = [0u8; 32];
        r.copy_from_slice(&self.0.public_key);
        Public(r)
    }

    /// Sign a message.
    fn sign(&self, message: &[u8]) -> Signature {
        let r = self.0.sign(message).expect("sign message failed");
        Signature::from_raw(&r)
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    fn verify<M: AsRef<[u8]>>(sig: &Self::Signature, message: M, pubkey: &Self::Public) -> bool {
        Self::verify_weak(&sig.0[..], message.as_ref(), pubkey)
    }

    /// Verify a signature on a message. Returns true if the signature is good.
    ///
    /// This doesn't use the type system to ensure that `sig` and `pubkey` are the correct
    /// size. Use it only if you're coming from byte buffers and need the speed.
    fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(sig: &[u8], message: M, pubkey: P) -> bool {
        wots_verify(message.as_ref(), sig, pubkey.as_ref()).is_ok()
    }

    /// Return a vec filled with raw data.
    fn to_raw_vec(&self) -> Vec<u8> {
        let seed = self.seed();
        seed.0.to_vec()
    }
}

#[cfg(feature = "full_crypto")]
impl Pair {
    /// Get the seed for this key.
    pub fn seed(&self) -> Seed {
        let mut s = [0; 64];

        let seed = [self.0.seed, self.0.p_seed].concat();
        s.copy_from_slice(seed.as_ref());

        Seed(s)
    }

    /// Exactly as `from_string` except that if no matches are found then, the the first 64
    /// characters are taken (padded with spaces as necessary) and used as the MiniSecretKey.
    #[cfg(feature = "std")]
    pub fn from_legacy_string(s: &str, password_override: Option<&str>) -> Pair {
        Self::from_string(s, password_override).unwrap_or_else(|_| {
            let mut bytes = [0; 64];
            bytes.copy_from_slice(s.as_bytes());
            Self::from_seed(&Seed::from(bytes))
        })
    }
}

impl CryptoType for Public {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}

impl CryptoType for Signature {
    #[cfg(feature = "full_crypto")]
    type Pair = Pair;
}

#[cfg(feature = "full_crypto")]
impl CryptoType for Pair {
    type Pair = Pair;
}
