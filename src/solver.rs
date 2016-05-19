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
