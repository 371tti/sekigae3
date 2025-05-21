use sekigae3::collector::{seat::{ForSortSeat, Seat}, user::{SeatPos, User, WantSeat}};
use rand::Rng;

fn main() {
    let n = 10; // 行数
    let m = 10; // 列数
    let user_count = n * m;

    let mut sekigae = ForSortSeat::new();
    let seat = Seat {
        structure: vec![vec![true; m]; n],
    };

    let mut users = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..user_count {
        let name = format!("User{}", i + 1);
        let mut want_seat = WantSeat::new();
        // 各ユーザーがランダムな座席を希望
        let row = rng.gen_range(0..n);
        let col = rng.gen_range(0..m);
        want_seat.add_pos(SeatPos::new(row, col, 1.0));
        let mut user = User::new(i, name);
        user.add_want(want_seat);
        users.push(user);
    }

    sekigae.init(seat, users);
    println!("Initial structure: {:?}", sekigae.structure);
    println!("befor_cost {}", sekigae.total_cost());
    sekigae.optimize();
    println!("After optimization: {:?}", sekigae.structure);
    println!("after_cost {}", sekigae.total_cost());
}
