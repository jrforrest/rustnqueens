use board::Board;
use pos::Pos;

#[derive(Clone)]

pub struct Solution {
    board: Board,
}

impl Solution {
    pub fn new() -> Solution {
        let mut board = Board::new(None);
        board.populate_random();
        Solution{board: board}
    }

    pub fn from_board(board: &Board) -> Solution {
        Solution{board: board.clone()}
    }

    pub fn solved(&self) -> bool {
        self.board.solved()
    }

    pub fn make_random_move(&mut self) {
        let queen = self.random_queen();

        self.board.move_to_random_col(&queen);
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    fn random_queen(&self) -> Pos {
        match self.board.random_piece() {
            Some(queen) => queen.clone(),
            None => panic!("Cannot move on random board")
        }
    }
}

#[test]
fn test_from_board() {
    let board = Board::new(Some(Dims{x: 5, y: 5}));
    let solution = Solution.from_board(board);

    assert!(solution);
}
