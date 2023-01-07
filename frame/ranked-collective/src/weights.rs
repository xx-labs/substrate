// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_ranked_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm2`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_ranked_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/ranked-collective/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ranked_collective.
pub trait WeightInfo {
	fn add_member() -> Weight;
	fn remove_member(r: u32, ) -> Weight;
	fn promote_member(r: u32, ) -> Weight;
	fn demote_member(r: u32, ) -> Weight;
	fn vote() -> Weight;
	fn cleanup_poll(n: u32, ) -> Weight;
}

/// Weights for pallet_ranked_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn add_member() -> Weight {
		// Minimum execution time: 24_344 nanoseconds.
		Weight::from_ref_time(24_856_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Minimum execution time: 36_881 nanoseconds.
		Weight::from_ref_time(39_284_238 as u64)
			// Standard Error: 16_355
			.saturating_add(Weight::from_ref_time(11_385_424 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(r as u64)))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Minimum execution time: 27_444 nanoseconds.
		Weight::from_ref_time(28_576_394 as u64)
			// Standard Error: 4_818
			.saturating_add(Weight::from_ref_time(519_056 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Minimum execution time: 36_539 nanoseconds.
		Weight::from_ref_time(39_339_893 as u64)
			// Standard Error: 16_526
			.saturating_add(Weight::from_ref_time(807_457 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:0)
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	// Storage: RankedCollective Voting (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote() -> Weight {
		// Minimum execution time: 50_548 nanoseconds.
		Weight::from_ref_time(51_276_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	// Storage: RankedCollective VotingCleanup (r:1 w:0)
	// Storage: RankedCollective Voting (r:0 w:2)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Minimum execution time: 16_222 nanoseconds.
		Weight::from_ref_time(22_982_955 as u64)
			// Standard Error: 3_863
			.saturating_add(Weight::from_ref_time(1_074_054 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	fn add_member() -> Weight {
		// Minimum execution time: 24_344 nanoseconds.
		Weight::from_ref_time(24_856_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn remove_member(r: u32, ) -> Weight {
		// Minimum execution time: 36_881 nanoseconds.
		Weight::from_ref_time(39_284_238 as u64)
			// Standard Error: 16_355
			.saturating_add(Weight::from_ref_time(11_385_424 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(r as u64)))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(r as u64)))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:0 w:1)
	// Storage: RankedCollective IdToIndex (r:0 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn promote_member(r: u32, ) -> Weight {
		// Minimum execution time: 27_444 nanoseconds.
		Weight::from_ref_time(28_576_394 as u64)
			// Standard Error: 4_818
			.saturating_add(Weight::from_ref_time(519_056 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:1)
	// Storage: RankedCollective MemberCount (r:1 w:1)
	// Storage: RankedCollective IdToIndex (r:1 w:1)
	// Storage: RankedCollective IndexToId (r:1 w:1)
	/// The range of component `r` is `[0, 10]`.
	fn demote_member(r: u32, ) -> Weight {
		// Minimum execution time: 36_539 nanoseconds.
		Weight::from_ref_time(39_339_893 as u64)
			// Standard Error: 16_526
			.saturating_add(Weight::from_ref_time(807_457 as u64).saturating_mul(r as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: RankedCollective Members (r:1 w:0)
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:1)
	// Storage: RankedCollective Voting (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote() -> Weight {
		// Minimum execution time: 50_548 nanoseconds.
		Weight::from_ref_time(51_276_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: RankedPolls ReferendumInfoFor (r:1 w:0)
	// Storage: RankedCollective VotingCleanup (r:1 w:0)
	// Storage: RankedCollective Voting (r:0 w:2)
	/// The range of component `n` is `[0, 100]`.
	fn cleanup_poll(n: u32, ) -> Weight {
		// Minimum execution time: 16_222 nanoseconds.
		Weight::from_ref_time(22_982_955 as u64)
			// Standard Error: 3_863
			.saturating_add(Weight::from_ref_time(1_074_054 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}
