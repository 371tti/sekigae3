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
    let users = vec![
        User::new(0, "Alice".to_string()),
        User::new(1, "Bob".to_string()),
        User::new(2, "Charlie".to_string()),
    ];
    sekigae.init(seat, users);
    println!("Initial structure: {:?}", sekigae.structure);
    println!("befor_cost {}", sekigae.total_cost());
}
