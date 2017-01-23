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
use rand::{thread_rng, Rng};

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
        let jep_piece = self.random_jeporadized_piece();
        self.board.move_to_random_col(&jep_piece);
        self.temp -= 1;
    }


   /// Selects a random piece that is in jeporady
   fn random_jeporadized_piece(&self) -> Pos {
       let jep: Vec<Pos> = self.board.jeporadized_pieces();
       let chosen = thread_rng().choose(jep.as_slice());

       match chosen {
           Some(piece) => {
            let new_piece: Pos = piece.clone();
            new_piece
           },
           None => panic!("Could not find jeporadized piece!")
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
