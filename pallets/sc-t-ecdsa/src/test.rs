// Copyright (C) 2020-2021 Polkadex OU
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// This is file is modified from multi-party-ecdsa Zen-Go library
use crate::mpc;
use curv::arithmetic::Converter;
use curv::elliptic::curves::traits::ECPoint;
use round_based::{IsCritical, StateMachine};
use serde::Serialize;
use serde_json;

#[test]
fn threshold_ecdsa_key_generation() {
    let mut parties = vec![];
    for i in 1..5 {
        parties.push(mpc::Keygen::new(i, 2, 4).unwrap())
    }
    loop {
        let mut msgs = vec![];
        for party in &mut parties {
            if party.wants_to_proceed() {
                // println!("Party {} wants to proceed", party.party_ind());
                // println!("  - before: {:?}", party);

                let round_old = party.current_round();

                match party.proceed() {
                    Ok(()) => (),
                    Err(err) if err.is_critical() => return panic!(err),
                    Err(err) => {
                        println!("Non-critical error encountered: {:?}", err);
                    }
                }
                let round_new = party.current_round();
                // println!(
                //     "Party {} send {} messages(s) ",
                //     party.party_ind(),
                //     party.message_queue().len()
                // );
            }
            msgs.append(party.message_queue())
        }

        for party in &mut parties {
            let party_i = party.party_ind();
            let msgs = msgs.iter().filter(|m| {
                m.sender != party_i && (m.receiver.is_none() || m.receiver == Some(party_i))
            });

            for msg in msgs {
                assert!(
                    !party.wants_to_proceed(),
                    "simulation is silly and doesn't expect party \
                         to wanna proceed at the middle of message handling"
                );
                // println!(
                //     "Party {} got message from={}, broadcast={}: {:?}",
                //     party.party_ind(),
                //     msg.sender,
                //     msg.receiver.is_none(),
                //     msg,
                // );
                println!(
                    "Size of Message in Bytes: {:?}",
                    serde_json::to_string(msg).unwrap().bytes().len()
                );
                // println!("  - before: {:?}", party);
                match party.handle_incoming(msg.clone()) {
                    Ok(()) => (),
                    Err(err) if err.is_critical() => panic!(err),
                    Err(err) => {
                        println!("Non-critical error encountered: {:?}", err);
                    }
                }
                // println!("  - after : {:?}", party);
            }
        }

        let is_finished = parties[0].is_finished();
        let same_answer_for_all_parties = parties.iter().all(|p| p.is_finished() == is_finished);
        assert!(same_answer_for_all_parties);

        if is_finished {
            let mut results = vec![];
            for party in &mut parties {
                results.push(
                    party
                        .pick_output()
                        .expect("is_finished == true, but pick_output == None")
                        .unwrap(),
                )
            }

            println!("Alice Public Key: {:?}", results[0].public_key());
            break;
        }
    }
}
