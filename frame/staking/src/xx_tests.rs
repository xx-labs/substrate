// This file is part of XX-Network.

// Added as part the code review and testing
// by ChainSafe Systems Aug 2021

//! Tests for modifications made to the staking module to support the XX network specific functionality

use super::*;
use frame_support::{
    assert_noop, assert_ok,
    traits::{Currency},
};
use frame_election_provider_support::ElectionProvider;
use mock::*;
use substrate_test_utils::assert_eq_uvec;

/////////////////////////////////////
//            CMIX ID              //
/////////////////////////////////////

#[test]
fn calling_bond_correctly_stores_cmix_id() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond with cmix ID
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));

            // confirm cmix ID is correctly stored
            assert!(CmixIds::<Test>::contains_key(&cmix_id(10u8).unwrap()));
        })
}

#[test]
fn calling_bond_with_existing_cmix_id_fails() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            let _ = Balances::make_free_balance_be(&20, stash_value);

            // ok to bond the first time
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));
            // if different account tries to bond, fails with ValidatorCmixIdNotUnique
            assert_noop!(
                Staking::bond(Origin::signed(20), 21, stash_value, cmix_id(10u8)),
                Error::<Test>::ValidatorCmixIdNotUnique,
                );
        })
}

#[test]
fn calling_validate_from_stash_with_no_cmix_id_fails() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond without cmix id
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, None));
            // if account tries to validate, fails with ValidatorMustHaveCmixId
            assert_noop!(
                Staking::validate(Origin::signed(11), Default::default()),
                Error::<Test>::ValidatorMustHaveCmixId,
                );
        })
}

#[test]
fn after_full_unbond_cmix_id_is_removed() {
    let stash_value = 100;
    ExtBuilder::default()
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond with cmix ID
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));

            // confirm cmix ID is correctly stored
            assert!(CmixIds::<Test>::contains_key(&cmix_id(10u8).unwrap()));

            mock::start_active_era(1);

            // unbond full amount
            assert_ok!(Staking::unbond(Origin::signed(11), stash_value));

            // trigger future era
            mock::start_active_era(5);

            // withdraw unbonded, which will kill stash
            assert_ok!(Staking::withdraw_unbonded(Origin::signed(11), 0));

            // confirm cmix ID is correctly removed
            assert!(!CmixIds::<Test>::contains_key(&cmix_id(10u8).unwrap()));
        })
}

#[test]
fn calling_set_cmix_id_correctly_stores_cmix_id() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // Can't call function if not bonded
            assert_noop!(
                Staking::set_cmix_id(Origin::signed(10), cmix_id(10u8).unwrap()),
                Error::<Test>::NotStash,
            );

            // Bond account without cmix id
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, None));

            // Can't call function from controller
            assert_noop!(
                Staking::set_cmix_id(Origin::signed(11), cmix_id(10u8).unwrap()),
                Error::<Test>::NotStash,
            );

            // Set cmix id
            assert_ok!(Staking::set_cmix_id(Origin::signed(10), cmix_id(10u8).unwrap()));

            // confirm cmix ID is correctly stored
            assert!(CmixIds::<Test>::contains_key(&cmix_id(10u8).unwrap()));
        })
}

#[test]
fn calling_set_cmix_id_stash_already_has_cmix_id_fails() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);

            // bond first validator with cmix id
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));

            // can't set cmix id, since it's already present
            assert_noop!(
                Staking::set_cmix_id(Origin::signed(10), cmix_id(11u8).unwrap()),
                Error::<Test>::StashAlreadyHasCmixId,
            );
        })
}

#[test]
fn calling_set_cmix_id_with_existing_cmix_id_fails() {
    let stash_value = 100;
    ExtBuilder::default()
        .has_stakers(false)
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            let _ = Balances::make_free_balance_be(&20, stash_value);

            // bond first validator with cmix id
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));

            // bond second validator without
            assert_ok!(Staking::bond(Origin::signed(20), 21, stash_value, None));

            // if second vaidator tries to set existing cmix id, fails with ValidatorCmixIdNotUnique
            assert_noop!(
                Staking::set_cmix_id(Origin::signed(20), cmix_id(10u8).unwrap()),
                Error::<Test>::ValidatorCmixIdNotUnique,
            );
        })
}

#[test]
fn transfer_cmix_id_works() {
    let stash_value = 100;
    ExtBuilder::default()
        .build_and_execute(|| {
            let _ = Balances::make_free_balance_be(&10, stash_value);
            let _ = Balances::make_free_balance_be(&12, stash_value);

            // Bond account 10 with cmix id
            assert_ok!(Staking::bond(Origin::signed(10), 11, stash_value, cmix_id(10u8)));
            // Bond account 12 without cmix id
            assert_ok!(Staking::bond(Origin::signed(12), 13, stash_value, None));

            // Confirm cmix ids
            assert_eq!(
                Staking::ledger(&11),
                Some(StakingLedger {
                    stash: 10,
                    total: 100,
                    active: 100,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    cmix_id: cmix_id(10u8)
                })
            );
            assert_eq!(
                Staking::ledger(&13),
                Some(StakingLedger {
                    stash: 12,
                    total: 100,
                    active: 100,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    cmix_id: None
                })
            );

            // Execute cmix id transfer
            assert_ok!(Staking::transfer_cmix_id(Origin::signed(10), 12));

            // Confirm cmix ID was correctly transferred
            assert_eq!(
                Staking::ledger(&11),
                Some(StakingLedger {
                    stash: 10,
                    total: 100,
                    active: 100,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    cmix_id: None
                })
            );
            assert_eq!(
                Staking::ledger(&13),
                Some(StakingLedger {
                    stash: 12,
                    total: 100,
                    active: 100,
                    unlocking: vec![],
                    claimed_rewards: vec![],
                    cmix_id: cmix_id(10u8)
                })
            );
        })
}

#[test]
fn check_transfer_cmix_id_errors() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Calling from a non stash account fails
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(10), 41),
                Error::<Test>::NotStash,
            );

            // Destination is not a stash account
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(11), 40),
                Error::<Test>::NotStash,
            );

            // Origin stash has no cmix id
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(41), 11),
                Error::<Test>::StashNoCmixId,
            );

            // Destination has cmix id
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(11), 21),
                Error::<Test>::StashAlreadyHasCmixId,
            );

            // Origin is validating
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(11), 41),
                Error::<Test>::StashValidating,
            );

            // Chill
            assert_ok!(Staking::chill(Origin::signed(10)));

            // Origin is active validator
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(11), 41),
                Error::<Test>::StashActiveValidator,
            );
        })
}

#[test]
fn check_transfer_cmix_id_election_ongoing() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Trigger the election
            let _ = <Test as Config>::ElectionProvider::elect().ok();

            // Election is ongoing
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(11), 41),
                Error::<Test>::ElectionOngoing,
            );
        })
}

#[test]
fn check_transfer_cmix_id_elected_validator() {
    ExtBuilder::default()
        .build_and_execute(|| {
            // Bond extra coins to get validator 31 elected
            assert_ok!(Staking::bond_extra(Origin::signed(31), 1000));

            // Advance sessions
            start_session(1);
            start_session(2);

            // Election happened, next validators should be available

            // Stop validating
            assert_ok!(Staking::chill(Origin::signed(30)));

            // Stash is elected validator
            assert_noop!(
                Staking::transfer_cmix_id(Origin::signed(31), 41),
                Error::<Test>::StashElectedValidator,
            );
        })
}

////////////////////////////////////////
//         Rewards Destination        //
////////////////////////////////////////

#[test]
fn rewards_paid_to_stash() {
    ExtBuilder::default()
        .build_and_execute(|| {
            let init_a_stash = Balances::total_balance(&11);
            let init_a_stake = 1000;
            let init_a_ctrl = Balances::total_balance(&10);

            let init_b_stash = Balances::total_balance(&21);
            let init_b_stake = 1000;
            let init_b_ctrl = Balances::total_balance(&20);

            let init_n_stash = Balances::total_balance(&101);
            let init_n_stake = 500;
            let init_n_ctrl = Balances::total_balance(&100);

            // set some reward points for validators
            // they all get a bonus 1 for being initialized
            <Pallet<Test>>::reward_by_ids(vec![(11, 80)]);
            <Pallet<Test>>::reward_by_ids(vec![(21, 20)]);

            let a_points = 80 + 1;
            let b_points = 20 + 1;

            assert_eq!(
                Staking::eras_reward_points(active_era()),
                EraRewardPoints {
                    total: a_points + b_points,
                    individual: vec![(11, a_points), (21, b_points)]
                        .into_iter()
                        .collect(),
                }
            );

            let total_payout_0 = current_total_payout_for_duration(reward_time_per_era());

            // advance to the next era and compute rewards for previous
            // there are 3 sessions per era in this test config so we are now in era 2
            start_session(3);

            // compute how much of the total era reward is allocated to each validator
            let a_part = Perbill::from_rational(a_points, a_points + b_points);
            let b_part = Perbill::from_rational(b_points, a_points + b_points);

            // Calculate how each validators share of total reward is allocated
            // between themselves and their nominators
            // Exposures for validators are their initial stash balances
            // Exposures for the nominator is their initial stash split between each nomination
            // (the vote allocation from nominator in this case is: 1/4 to A, 3/4 to B)

            let n_vote_to_a = init_n_stake / 4;
            let n_vote_to_b = init_n_stake * 3 / 4;

            let a_exp_part =
                Perbill::from_rational::<u32>(init_a_stake, init_a_stake + n_vote_to_a);
            let b_exp_part =
                Perbill::from_rational::<u32>(init_b_stake, init_b_stake + n_vote_to_b);
            let n_from_a_part =
                Perbill::from_rational::<u32>(n_vote_to_a, init_a_stake + n_vote_to_a);
            let n_from_b_part =
                Perbill::from_rational::<u32>(n_vote_to_b, init_b_stake + n_vote_to_b);
            // now we have everything we need to compute rewards
            let a_rewards = a_part * a_exp_part * total_payout_0;
            let b_rewards = b_part * b_exp_part * total_payout_0;
            let n_rewards =
                a_part * n_from_a_part * total_payout_0 + b_part * n_from_b_part * total_payout_0;

            make_all_reward_payment(0);

            // Rewards paid to stash accounts
            assert_eq!(Balances::total_balance(&11), init_a_stash + a_rewards,);
            assert_eq!(Balances::total_balance(&21), init_b_stash + b_rewards,);
            assert_eq!(Balances::total_balance(&101), init_n_stash + n_rewards,);

            // Controller accounts remain the same
            assert_eq!(Balances::total_balance(&10), init_a_ctrl,);
            assert_eq!(Balances::total_balance(&20), init_b_ctrl,);
            assert_eq!(Balances::total_balance(&100), init_n_ctrl,);

            //////////////////// second era /////////////////////////

            // We expect that identical rewards will be paid out again
            // if the same reward points are used

            <Pallet<Test>>::reward_by_ids(vec![(11, 80)]);
            <Pallet<Test>>::reward_by_ids(vec![(21, 20)]);

            start_session(6);

            make_all_reward_payment(1);

            assert_eq!(
                Balances::total_balance(&11),
                init_a_stash + a_rewards + a_rewards,
            );
            assert_eq!(
                Balances::total_balance(&21),
                init_b_stash + b_rewards + b_rewards,
            );
            assert_eq!(
                Balances::total_balance(&101),
                init_n_stash + n_rewards + n_rewards,
            );
        })
}

////////////////////////////////////////
//           Custody Accounts         //
////////////////////////////////////////

#[test]
fn can_set_custody_accounts_in_builder() {
    let custody_account_id = 10;
    ExtBuilder::default()
        .custody_accounts(&[custody_account_id])
        .build_and_execute(|| {
            assert!(<Test as Config>::CustodyHandler::is_custody_account(
                &custody_account_id
            ));
            assert!(
                <Test as Config>::CustodyHandler::is_custody_account(&(custody_account_id + 1))
                    == false
            );
        })
}

#[test]
fn exposure_not_counted_for_custody_accounts() {
    let a = 101;

    ExtBuilder::default()
        .nominate(false)
        .custody_accounts(&[a]) // A is a custody account
        .build_and_execute(|| {
            // Starts with 11 and 21 as validators
            // 31 is a validator with lower stake, so not elected
            assert_eq_uvec!(validator_controllers(), vec![10, 20]);

            // No nominators, so all others exposure is empty
            assert!(ErasStakers::<Test>::iter_prefix_values(active_era())
                .all(|exposure| exposure.others.is_empty()));

            // Custody account nominates 11 and 31
            assert_ok!(Staking::bond(Origin::signed(101), 100, 2000, None));
            assert_ok!(Staking::nominate(Origin::signed(100), vec![11, 31]));

            // Start next era, electing 11 and 31
            mock::start_active_era(1);
            assert_eq_uvec!(validator_controllers(), vec![10, 30]);

            // However, custody stake is not exposed, but helped elect 31
            assert!(ErasStakers::<Test>::iter_prefix_values(active_era())
                .all(|exposure| exposure.others.is_empty()));
        })
}

#[test]
fn custody_accounts_cannot_be_slashed() {
    let a = 11;
    let b = 21;
    let c = 101;

    ExtBuilder::default()
        .custody_accounts(&[c]) // c is a custody account
        .build_and_execute(|| {

            assert_eq!(Balances::free_balance(a), 1000);
            assert_eq!(Balances::free_balance(b), 2000);
            assert_eq!(Balances::free_balance(c), 2000);

            let initial_balance = Balances::free_balance(c);

            add_slash(&a);
            add_slash(&b);

            assert_eq!(
                Balances::free_balance(&c),
                initial_balance,
            );

        })
}

#[test]
fn non_custody_accounts_can_be_slashed() {
    let a = 11;
    let b = 21;
    let c = 101;

    ExtBuilder::default()
        // no custody accounts
        .build_and_execute(|| {

            assert_eq!(Balances::free_balance(a), 1000);
            assert_eq!(Balances::free_balance(b), 2000);
            assert_eq!(Balances::free_balance(c), 2000);

            add_slash(&a);
            add_slash(&b);

            assert_eq!(
                Balances::free_balance(&c),
                1951, // new slashed balance
            );

        })
}

//////////////////////////////////////////
//         Rewards Points System        //
//////////////////////////////////////////

#[test]
fn can_set_block_points_in_builder() {
    let points = 7;
    ExtBuilder::default()
        .block_points(points)
        .build_and_execute(|| {
            assert_eq!(
                <Test as Config>::CmixHandler::get_block_points(),
                points
            );
        })
}

#[test]
fn validators_are_always_initialized_with_one_point() {
    let a = 11;
    ExtBuilder::default().build_and_execute(|| {
        // assigned 0 points a few times
        <Pallet<Test>>::reward_by_ids(vec![(a, 0), (a, 0)]);
        // results in 1 point from the first initialization
        assert_eq!(
            Staking::eras_reward_points(active_era()),
            EraRewardPoints {
                total: 1,
                individual: vec![(a, 1)].into_iter().collect()
            }
        )
    })
}

#[test]
fn can_deduct_points() {
    let a = 11;
    let b = 21;
    ExtBuilder::default().build_and_execute(|| {
        // add points first
        <Pallet<Test>>::reward_by_ids(vec![(a, 5), (b, 7)]);
        assert_eq!(
            Staking::eras_reward_points(active_era()),
            EraRewardPoints {
                total: 14,
                individual: vec![(a, 5 + 1), (b, 7 + 1)].into_iter().collect()
            }
        );

        // deduct some from each
        Staking::deduct_by_ids(vec![(a, 2), (b, 3)]);
        assert_eq!(
            Staking::eras_reward_points(active_era()),
            EraRewardPoints {
                total: 9,
                individual: vec![(a, 3 + 1), (b, 4 + 1)].into_iter().collect()
            }
        );
    })
}

#[test]
fn cannot_deduct_below_one() {
    let a = 11;
    ExtBuilder::default().build_and_execute(|| {
        // add points first
        <Pallet<Test>>::reward_by_ids(vec![(a, 5)]);
        assert_eq!(
            Staking::eras_reward_points(active_era()),
            EraRewardPoints {
                total: 6,
                individual: vec![(a, 6)].into_iter().collect()
            }
        );

        // deduct more points than a has
        Staking::deduct_by_ids(vec![(a, 10)]);
        assert_eq!(
            Staking::eras_reward_points(active_era()),
            EraRewardPoints {
                // it keeps one
                total: 1,
                individual: vec![(a, 1)].into_iter().collect()
            }
        );
    })
}

//////////////////////////////////////////
//             Rewards Pool             //
//////////////////////////////////////////

#[test]
fn reward_handler_called_on_do_payout_stakers() {
   ExtBuilder::default()
        .build_and_execute(|| {
            let init_11 = Balances::total_balance(&11);

            // give 11 some points
            <Pallet<Test>>::reward_by_ids(vec![(11, 80)]);

            let total_payout_0 = current_total_payout_for_duration(reward_time_per_era());

            // there are 3 sessions per era in this test config so we are now in era 2
            start_session(3);
            make_all_reward_payment(0);

            assert!(Balances::total_balance(&11) > init_11);
            assert_eq!(mock::RewardMock::total(), total_payout_0);
        })
}


//////////////////////////////////////////
//       Min Validator Commission       //
//////////////////////////////////////////

#[test]
fn min_validator_commission_check_works() {
    ExtBuilder::default()
        .existential_deposit(100)
        .balance_factor(100)
        .min_validator_commission(Perbill::from_percent(2))
        .build_and_execute(|| {
            assert_ok!(Staking::bond(Origin::signed(3), 4, 500, cmix_id(3u8)));
            // commission lower than allowed is not enough to be a validator
            assert_noop!(
				Staking::validate(
				    Origin::signed(4),
				    ValidatorPrefs { commission: Perbill::from_percent(1), blocked: false }
				),
				Error::<Test>::ValidatorCommissionTooLow,
			);

            // 2 percent is exactly enough
            assert_ok!(
				Staking::validate(
				    Origin::signed(4),
				    ValidatorPrefs { commission: Perbill::from_percent(2), blocked: false }
				)
			);

            // chill
            assert_ok!(Staking::chill(Origin::signed(4)));

            // >2 percent is also enough
            assert_ok!(
				Staking::validate(
				    Origin::signed(4),
				    ValidatorPrefs { commission: Perbill::from_percent(3), blocked: false }
				)
			);
        })
}
