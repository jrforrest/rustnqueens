/// This solver will be using a simulated annealing, run in a parallel cluster
/// configuration (see: http://dos.iitm.ac.in/LabPapers/parallelSAJPDC.pdf).
///
/// Basically, each cluster starts with a random solution, and the nodes
/// re-converge each N iterations with the best current solution.
/// 
/// Since we have an acceptance measure here (that being a board which is
/// solved) we can re-converge to perpetuity, yielding a perfect solution when
/// any one node has identified a satisfactory solution.

use board::Board;
use pos::Pos;

pub struct Solver {
    board: Board,
    temp: u32,
}

impl Solver {
    pub fn new() -> Solver {
        let mut board = Board::new(None);
        board.populate_random();
        Solver{board: board, temp: 10000}
    }

    pub fn solve(&mut self) {
        loop {
            if self.board.solved() || self.temp == 0 {
                break;
            }

            self.step();
       }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Steps the simulation one step
    pub fn step(&mut self) {
        let queen = self.random_queen();

        self.board.move_to_random_col(&queen);
        self.temp -= 1;
    }

    fn random_queen(&self) -> Pos {
        match self.board.random_piece() {
            Some(queen) => queen.clone(),
            None => panic!("Cannot step empty board!")
        }
    }
}

#[test]
fn test_solve() {
    let mut solver = Solver::new();
    solver.solve();
    println!("{}", solver.board);
    assert!(solver.board.solved());

}
