use std::collections::HashMap;

use sekigae3::collector::{seat::{ForSortSeat, Seat}, user::{SeatPos, User, WantSeat}};

fn main() {
    let mut sekigae = ForSortSeat::new();
    let seat = Seat {
        structure:
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, false, true],
            ],
    };
    let mut want_seat_alice = WantSeat::new();
    want_seat_alice.add_pos(SeatPos::new(2, 2, 1.0));
    let mut alice = User::new(0, "Alice".to_string());
    alice.add_want(want_seat_alice);

    let mut want_seat_bob = WantSeat::new();
    want_seat_bob.add_pos(SeatPos::new(2, 2, 1.0));
    let mut bob = User::new(1, "Bob".to_string());
    bob.add_want(want_seat_bob);

    let users = vec![
        alice,
        bob,
        User::new(2, "Charlie".to_string()),
    ];
    sekigae.init(seat, users);
    println!("Initial structure: {:?}", sekigae.structure);
    println!("befor_cost {}", sekigae.total_cost());
    sekigae.optimization();
    println!("After optimization: {:?}", sekigae.structure);
    println!("after_cost {}", sekigae.total_cost());
}
