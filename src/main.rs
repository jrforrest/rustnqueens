extern crate rand;

mod pos;
mod board;
mod solver;

use solver::Solver;

fn main() {
    let mut solver = Solver::new();

    for _ in 0..10 {
        println!("{}", solver.get_board());
        solver.step();
    }
}
