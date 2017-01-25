/// This solver will be using a simulated annealing, run in a parallel cluster
/// configuration (see: http://dos.iitm.ac.in/LabPapers/parallelSAJPDC.pdf).
///
/// Basically, each cluster starts with a random solution, and the nodes
/// re-converge each N iterations with the best current solution.
///
/// Since we have an acceptance measure here (that being a board which is
/// solved) we can re-converge to perpetuity, yielding a perfect solution when
/// any one node has identified a satisfactory solution.

extern crate rand;

use solution::Solution;
use board::Board;
use std::rc::Rc;

pub struct Solver {
    solution: Rc<Solution>,
    best_solution: Rc<Solution>,
    temp: f32,
    cooling_rate: f32,
}

impl Solver {
    pub fn new() -> Solver {
        let solution = Rc::new(Solution::new());

        Solver{
            solution: solution.clone(),
            best_solution: solution.clone(),
            temp: 1000f32,
            cooling_rate: 0.03,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.solution.get_board()
    }

    /// Steps the simulation one step
    pub fn step(&mut self) {
        let new_solution = self.solution.get_neighbor_solution();

        if self.should_accept(new_solution.n_jeporadized_queens() as f32) {
            self.solution = Rc::new(new_solution);

            let (new, old) = (self.solution.n_jeporadized_queens(),
                self.best_solution.n_jeporadized_queens());

            if new < old {
                self.best_solution = self.solution.clone()
            }
        }

        self.cool();
    }

    fn cool(&mut self) {
        self.temp *= 1.0 - self.cooling_rate;
    }

    fn should_accept(&self, new_energy: f32) -> bool {
        let threshold = rand::random::<f32>();

        self.acceptance_probability(new_energy) > threshold
    }

    fn acceptance_probability(&self, new_energy: f32) -> f32 {
        let old_energy = self.solution.n_jeporadized_queens() as f32;

        if new_energy < old_energy {
            1.0
        } else {
            ((old_energy - new_energy) / self.temp).exp()
        }
    }
}
