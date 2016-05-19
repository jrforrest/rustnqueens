use pos::*;
use std::collections::HashSet;
use std::option::Option;

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

    pub fn jeporadized_pieces(&self) -> Vec<&Pos> {
        self.queens.iter().filter(|q| {
            self.queens.iter().any(|oq| q.can_attack(oq))
        }).collect()
    }

    pub fn populate_random(&mut self) {
        for y in 1..(self.dims.y + 1) {
            let x = random::<i32>() % self.dims.x + 1;
            self.add_queen(y, x);
        }
    }

    pub fn solved(&self) -> bool {
        self.jeporadized_pieces().len() == 0
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
