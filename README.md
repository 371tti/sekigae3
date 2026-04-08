# Sekigae3
座席最適化システムすね  
ユーザーに座りたい席のアンケートをとり、座席配置を最適化します

## library usage
`sekigae3` は席替え用の割り当て最適化エンジンてすてす。

```rust
use sekigae3::{ILSA, Problem, Seat};

let seats = vec![Seat { x: 0, y: 0 }];
let want_seats = vec![vec![(0u16, 1.0f32)]];
let pair_edges = vec![Vec::<(u16, f32)>::new()];

let problem = Problem::new(seats, want_seats, pair_edges);
let mut solver = ILSA::new(&problem, 42);
let best = solver.solve(10);

println!("cost = {}", best.cost());
```
