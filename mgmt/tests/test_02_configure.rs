#![allow(non_snake_case)]
#[macro_use] extern crate kukumba;
#[macro_use] mod helpers; use helpers::{harness, mock_env, tx};

use cosmwasm_std::{StdError, HumanAddr, Uint128, HandleResponse};
use sienna_mgmt::msg::Handle;
use sienna_schedule::{Schedule, schedule, pool, release_immediate_multi, allocation_addr};

kukumba!(

    #[configure]

    given "a contract" {
        harness!(deps; ALICE, BOB, MALLORY);
    }
    when "anyone but the admin tries to set a configuration" {
        todo!();
    } then "that fails" {
        todo!();
    }
    when "the admin tries to set a configuration that doesn't add up" {
        let s1 = schedule(100u128, vec![]);
    } then "that fails" {
        test_tx!(deps, ALICE, 0, 0;
            Handle::Configure { schedule: s1.clone() } =>
            tx_err!("schedule: pools add up to 0, expected 100"));
    }
    when "the admin tries to set a configuration that doesn't divide evenly" {
        let s1 = schedule(100u128, vec![]);
    } then "that fails" {
        todo!();
    }
    when "the admin tries to set a valid configuration" {
        let s1 = schedule(100, vec![
            pool("", 10, vec![
                release_immediate_multi(10, vec![
                    allocation_addr(10, &BOB)
                ])
            ]),
            pool("", 90, vec![
                release_immediate_multi(45, vec![
                    allocation_addr(45, &BOB)
                ]),
                release_immediate_multi(45, vec![
                    allocation_addr( 5, &BOB),
                    allocation_addr(10, &BOB),
                    allocation_addr(30, &BOB)
                ])
            ])
        ])
        test_tx!(deps, ALICE, 0, 0;
            Handle::Configure { schedule: s1.clone() } => tx_ok!());
    } then "the configuration is updated" {
        let pools = s1.pools.clone();
        test_q!(deps, Schedule;
            Schedule {
                schedule: Some(Schedule { total: s1.total, pools })
            });
    }
    when "someone else tries to set a valid configuration" {
        test_tx!(deps, MALLORY, 0, 0;
            Handle::Configure { schedule: schedule(0, vec![]) } =>
                tx_err_auth!());
    } then "the configuration remains unchanged" {
        let pools = s1.pools.clone();
        test_q!(deps, Schedule;
            Schedule {
                schedule: Some(Schedule { total: s1.total, pools })
            });
    }

);
