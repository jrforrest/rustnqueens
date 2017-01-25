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

    pub fn make_random_move(&mut self) {
        let queen = self.random_queen();

        self.board.move_to_random_col(&queen);
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn n_jeporadized_queens(&self) -> usize {
        self.board.jeporadized_pieces().len()
    }

    pub fn get_neighbor_solution(&self) -> Solution {
        let mut new = self.clone();
        new.make_random_move();
        new
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
    let board = Board::new(None);
    let solution = Solution::from_board(&board);

    assert!(true);
}

#[test]
fn test_get_neighbor_solution() {
    let solution = Solution::new();
    let new = solution.get_neighbor_solution();

    assert!(true);
}
