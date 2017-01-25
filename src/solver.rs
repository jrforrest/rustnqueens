extern crate rand;

use solution::Solution;
use board::Board;
use std::rc::Rc;

pub struct Solver {
    solution: Rc<Solution>,
    best_solution: Rc<Solution>,
    pub temp: f32,
    pub iteration: u32,
    cooling_factor: f32,
}

impl Solver {
    pub fn new() -> Solver {
        let solution = Rc::new(Solution::new());

        Solver{
            solution: solution.clone(),
            best_solution: solution.clone(),
            temp: 10000f32,
            iteration: 0,
            cooling_factor: 0.97f32,
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
        self.iteration += 1;
    }

    pub fn n_jeporadized_queens(&self) -> usize {
        self.solution.n_jeporadized_queens()
    }

    pub fn best_n_jeporadized_queens(&self) -> usize {
        self.best_solution.n_jeporadized_queens()
    }

    fn cool(&mut self) {
        self.temp *= self.cooling_factor;
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
