/// This solver will be using a simulated annealing, run in a parallel cluster
/// configuration (see: http://dos.iitm.ac.in/LabPapers/parallelSAJPDC.pdf).
///
/// Basically, each cluster starts with a random solution, and the nodes
/// re-converge each N iterations with the best current solution.
///
/// Since we have an acceptance measure here (that being a board which is
/// solved) we can re-converge to perpetuity, yielding a perfect solution when
/// any one node has identified a satisfactory solution.

use solution::Solution;
use board::Board;

pub struct Solver {
    solution: Solution,
    temp: u32,
}

impl Solver {
    pub fn new() -> Solver {
        let solution = Solution::new();
        Solver{solution: solution, temp: 10000}
    }

    pub fn solve(&mut self) {
        loop {
            if self.solution.solved() || self.temp == 0 {
                break;
            }

            self.step();
       }
    }

    pub fn get_board(&self) -> &Board {
        &self.solution.get_board()
    }

    /// Steps the simulation one step
    pub fn step(&mut self) {
        self.solution = self.solution.get_neighbor_solution();
        self.temp -= 1;
    }
}
