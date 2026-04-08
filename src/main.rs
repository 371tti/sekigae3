use sekigae3::{ILSA, Problem, Seat};

fn build_simple_problem() -> Problem {
    let seats = vec![
        Seat { x: 0, y: 0 },
        Seat { x: 1, y: 0 },
        Seat { x: 2, y: 0 },
        Seat { x: 0, y: 1 },
        Seat { x: 1, y: 1 },
        Seat { x: 2, y: 1 },
        Seat { x: 0, y: 2 },
        Seat { x: 1, y: 2 },
        Seat { x: 2, y: 2 },
    ];

    // want_seats[student] = [(seat_id, weight)]
    let want_seats = (0..9).map(|i| vec![]).collect(); // 全員無関心 ペア整合の確認用
    // let want_seats = vec![
    //     vec![(8, 1.0), (4, 0.7)],           // 学生0は席0が一番好きで、席4もまあまあ好き
    //     vec![(7, 1.0), (4, 0.6)],           // 学生1は席1が一番好きで、席4もまあまあ好き
    //     vec![(6, 1.0), (4, 0.6)],           // 学生2は席2が一番好きで、席4もまあまあ好き
    //     vec![(5, 1.0), (4, 0.7)],           // 学生3は席3が一番好きで、席4もまあまあ好き
    //     vec![(4, 1.0), (0, 0.4), (8, 0.4)], // 学生4は席4が一番好きで、席0と席8もまあまあ好き
    //     vec![(3, 1.0), (4, 0.7)],           // 学生5は席5が一番好きで、席4もまあまあ好き
    //     vec![(2, 1.0), (4, 0.6)],           // 学生6は席6が一番好きで、席4もまあまあ好き
    //     vec![(1, 1.0), (4, 0.6)],           // 学生7は席7が一番好きで、席4もまあまあ好き
    //     vec![(0, 1.0), (4, 0.7)],           // 学生8は席8が一番好きで、席4もまあまあ好き
    // ]; // つまり逆順 ペア整合より重みつよいので優先

    // pair_edges[student] = [(other_student_id, weight)]
    // a < b の辺だけを入れる簡易定義
    let pair_edges = vec![
        vec![(1, 0.8), (3, 0.7), (4, 1.0)], // 学生0は学生1と3と4と仲が良い
        vec![(2, 0.8), (4, 0.9)],           // 学生1は学生2と4と仲が良い
        vec![(5, 0.7), (4, 0.9)],           // 学生2は学生5と4と仲が良い
        vec![(6, 0.8), (4, 0.9)],           // 学生3は学生6と4と仲が良い
        vec![(5, 0.9), (7, 0.9), (8, 0.8)], // 学生4は学生5と7と8と仲が良い
        vec![(8, 0.8)],                     // 学生5は学生8と仲が良い
        vec![(7, 0.8)],                     // 学生6は学生7と仲が良い
        vec![(8, 0.8)],                     // 学生7は学生8と仲が良い
        vec![],                             // 学生8は特に仲の良い学生がいない (*'▽') < ﾎﾞｯﾁﾎﾞｯﾁﾎﾞｯﾁｰ
    ];

    Problem::new(seats, want_seats, pair_edges)
}

fn main() {
    let problem = build_simple_problem();

    let seat_count = problem.seat_count();

    let mut ilsa = ILSA::new(&problem, 42);
    let best = ilsa.solve(seat_count);

    println!("best cost: {:.3}", best.cost());
    println!("seat -> student: {:?}", best.by_seat());
}
