use curv::arithmetic::Converter;
use curv::elliptic::curves::traits::ECPoint;
use round_based::StateMachine;
use serde::Serialize;

use crate::worker;

#[test]
fn threshold_ecdsa_key_generation() {
    let mut alice = worker::Keygen::new(1, 2, 3).unwrap();
    let mut bob = worker::Keygen::new(2, 2, 3).unwrap();
    let mut charlie = worker::Keygen::new(3, 2, 3).unwrap();

    println!("Alice {:?}", alice);
    println!("Bob {:?}", bob);
    println!("Charlie {:?}", charlie);

    alice.proceed();
    bob.proceed();
    charlie.proceed();
    println!("Alice {:?}", alice);
    println!("Bob {:?}", bob);
    println!("Charlie {:?}", charlie);
}
