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

pub struct Solver {
    board: Board,
}


impl Solver {
    pub fn new() -> Solver {
        Solver{board: Board::new(None)}
    }
}

#[test]
fn test_new() {
    let solver = Solver::new();
    assert!(true);
}
