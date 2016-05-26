use pos::*;
use std::collections::HashSet;
use std::option::Option;
use std::fmt;

use rand::random;

#[derive(Clone)]
pub struct Board {
    queens: HashSet<Pos>,
    dims: Dims,
}

#[derive(Clone, Copy)]
pub struct Dims {
    x: i32,
    y: i32,
}

impl Board {
    pub fn new(given_dims: Option<Dims>) -> Board {
        let dims = match given_dims {
            None => Dims{x: 10, y: 10},
            Some(d) => d
        };

        Board { queens: HashSet::new(), dims: dims }
    }

    pub fn add_queen (&mut self, x: i32, y: i32) {
        self.queens.insert(Pos::new(x, y));
    }

    pub fn jeporadized_pieces(&self) -> Vec<Pos> {
        self.queens.iter().
            filter(|q| { self.queens.iter().any(|oq| q.can_attack(oq))}).
            map(|q| { q.clone() } ).
            collect()
    }

    pub fn populate_random(&mut self) {
        for x in 1..(self.dims.x + 1) {
            let y = self.random_col();
            self.add_queen(x, y);
        }
    }

    pub fn solved(&self) -> bool {
        self.jeporadized_pieces().len() == 0
    }

    pub fn move_to_random_col(&mut self, queen: &Pos) {
        self.queens.remove(&queen);

        // Shift the column by one if we collided with the column
        // of the given queen
        let mut y = self.random_col();
        if y == queen.y {
            y = self.shift_col(y);
        }

        self.add_queen(queen.x, y);
    }

    /// Returns a random column on the board
    fn random_col(&self) -> i32 {
        random::<i32>() % self.dims.y + 1
    }

    /// Shifts the given col number by one wrapping if necessary
    fn shift_col(&self, col: i32) -> i32 {
        (col + 1) % self.dims.y + 1
    }

    pub fn queen_at(&self, pos: Pos) -> bool {
        for q in self.queens.iter() {
            if *q == pos {
                return true;
            }
        }

        return false;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..(self.dims.x) {
            for y in 0..(self.dims.y) {
                let ch = if self.queen_at(Pos{x: x, y: y}) {
                    'x'
                } else {
                    '~'
                };

                try!(write!(f, "{}", ch));
            }

            try!(write!(f, "\n"));
        }

        Ok(())
    }
}

#[test]
fn test_add_queen() {
    let mut board = Board::new(None);
    board.add_queen(1,1);
}

#[test]
fn test_jeporadized_pieces() {
    let mut board = Board::new(None);
    board.add_queen(1,1);
    board.add_queen(2,2);

    assert!(board.jeporadized_pieces().len() == 2);
}

#[test]
fn test_default_size() {
    let board = Board::new(None);
    assert!(board.dims.x == 10);
    assert!(board.dims.y == 10);
}

#[test]
fn test_populate_random() {
    let mut board = Board::new(None);
    board.populate_random();
    assert!(board.queens.len() == 10);
}

#[test]
fn test_solved() {
    let mut board = Board::new(Some(Dims{x: 5, y: 5}));
    board.add_queen(1,1);
    board.add_queen(3,4);
    assert!(board.solved());
}
