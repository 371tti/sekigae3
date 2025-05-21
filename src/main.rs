use std::collections::HashMap;

use sekigae3::collector::{seat::{ForSortSeat, Seat}, user::{SeatPos, User, WantSeat}};

fn main() {
    let mut sekigae = ForSortSeat::new();
    let seat = Seat {
        structure: vec![
            vec![true, true, true, true, true],
            vec![true, true, true, true, true],
            vec![true, true, true, true, true],
            vec![true, true, true, true, true],
            vec![true, true, true, true, true],
        ],
    };

    let mut users = Vec::new();
    for i in 0..20 {
        let name = format!("User{}", i + 1);
        let mut want_seat = WantSeat::new();
        // それぞれのユーザーがランダムな座席を希望する例
        let row = (i % 5) as usize;
        let col = ((i * 3) % 5) as usize;
        want_seat.add_pos(SeatPos::new(row, col, 1.0));
        let mut user = User::new(i, name);
        user.add_want(want_seat);
        users.push(user);
    }

    sekigae.init(seat, users);
    println!("Initial structure: {:?}", sekigae.structure);
    println!("befor_cost {}", sekigae.total_cost());
    sekigae.optimization();
    println!("After optimization: {:?}", sekigae.structure);
    println!("after_cost {}", sekigae.total_cost());
}
