use sekigae3::solver::{ILSA, Problem, Seat};

fn build_demo_problem() -> Problem {
    let mut seats = Vec::new();

    for y in 0..30 {
        for x in 0..30 {
            let is_border = x == 0 || y == 0 || x == 29 || y == 29;
            let is_aisle = x % 7 == 0 || y % 6 == 0;
            let blocked_core_a = (8..14).contains(&y) && (10..18).contains(&x);
            let blocked_core_b = (16..23).contains(&y) && (4..11).contains(&x);
            let sparse_pattern = (x + y) % 3 == 0;
            let keep_sparse_lane = y % 5 == 0;

            let available = !is_border
                && !is_aisle
                && !blocked_core_a
                && !blocked_core_b
                && (!sparse_pattern || keep_sparse_lane);

            if available {
                seats.push(Seat {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
    }

    let n = seats.len();
    assert!(n > 0, "at least one seat is required");

    let front_zone: Vec<u16> = seats
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| (s.y <= 7).then_some(idx as u16))
        .collect();
    let window_zone: Vec<u16> = seats
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| (s.x <= 4 || s.x >= 24).then_some(idx as u16))
        .collect();
    let center_zone: Vec<u16> = seats
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            ((11..=18).contains(&s.x) && (10..=20).contains(&s.y)).then_some(idx as u16)
        })
        .collect();

    let mut want_seats = vec![Vec::<u16>::new(); n];
    for (student, wants) in want_seats.iter_mut().enumerate() {
        let anchor = (student * 97 + 31) % n;
        wants.push(anchor as u16);
        wants.push(((anchor + n / 3) % n) as u16);

        let zone = match student % 3 {
            0 => &front_zone,
            1 => &window_zone,
            _ => &center_zone,
        };
        if !zone.is_empty() {
            wants.push(zone[student % zone.len()]);
        }

        wants.sort_unstable();
        wants.dedup();
    }

    let mut pair_edges = vec![Vec::<(u16, f32)>::new(); n];

    let mut add_pair = |a: usize, b: usize, w: f32| {
        if a == b {
            return;
        }
        if pair_edges[a].iter().any(|&(other, _)| other as usize == b) {
            return;
        }
        pair_edges[a].push((b as u16, w));
        pair_edges[b].push((a as u16, w));
    };

    for i in 0..n {
        let near = (i + 1) % n;
        let rowmate = (i + 29) % n;
        let randomish = (i * 37 + 11) % n;
        add_pair(i, near, 1.0);
        add_pair(i, rowmate, 0.8);
        add_pair(i, randomish, 0.6);
    }

    Problem::new(seats, want_seats, pair_edges)
}

fn main() {
    let problem = build_demo_problem();
    println!("students/seats: {}", problem.student_count());

    let mut ilsa = ILSA::new(&problem, 0);
    let candidates = ilsa.solve_candidates(200, 5);

    for (idx, cand) in candidates.iter().enumerate() {
        println!("candidate #{}", idx + 1);
        println!("  cost: {:.3}", cand.cost());
        println!("  seat -> student: {:?}", cand.by_seat());
    }
}
