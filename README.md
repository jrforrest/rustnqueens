# RustNQueens

Rust practice: solves N queens problem on a 15x15 board without using the
filthy cheater's polynomial time algorithm.

### Building

`cargo build` should do the job.  Originally built with cargo
`0.9.0` and rustc `1.8.0`.

### Running

`cargo run` or just execute the executable produced via `cargo build`.

The underlying TUI lib segfaults with small window sizes so make sure your
term is large enough for at least 40x20 or so.

`s` key steps the simulated annealing algo, `q` key quits.
