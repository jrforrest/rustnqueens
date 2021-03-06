use pos::Pos;
use std::option::Option;
use std::vec::Vec;
use rand::{thread_rng, Rng};

use rand::random;

pub type Queen = Pos;

#[derive(Clone)]
pub struct Board {
    queens: Vec<Queen>,
    pub dims: Dims,
}

#[derive(Clone, Copy)]
pub struct Dims {
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn new(given_dims: Option<Dims>) -> Board {
        let dims = match given_dims {
            None => Dims{x: 15, y: 15},
            Some(d) => d
        };

        Board { queens: Vec::new(), dims: dims }
    }

    pub fn add_queen (&mut self, x: usize, y: usize) {
        self.queens.push(Queen::new(x, y));
    }

    pub fn jeporadized_pieces(&self) -> Vec<Queen> {
        self.queens.iter().
            filter(|q| { self.queens.iter().any(|oq| q.can_attack(oq))}).
            map(|q| { q.clone() } ).
            collect()
    }

    pub fn populate_random(&mut self) {
        for y in 0..(self.dims.x - 1) {
            let x = self.random_col();
            self.add_queen(x, y);
        }
    }

    #[allow(dead_code)]
    pub fn solved(&self) -> bool {
        self.jeporadized_pieces().len() == 0
    }

    pub fn move_to_random_col(&mut self, queen: &Queen) {
        self.remove_queen(&queen);

        // Shift the column by one if we collided with the column
        // of the given queen
        let mut x = self.random_col();
        if x == queen.x {
            x = self.shift_col(x);
        }

        self.add_queen(x, queen.y);
    }

    pub fn random_piece(&self) -> Option<&Queen> {
        thread_rng().choose(&self.queens)
    }

    fn remove_queen(&mut self, queen: &Queen) {
        let pos = self.queens.iter().position(|q| q == queen).unwrap();
        self.queens.remove(pos);
    }

    /// Returns a random column on the board
    fn random_col(&self) -> usize {
        random::<usize>() % (self.dims.y - 1)
    }

    /// Shifts the given col number by one wrapping if necessary
    fn shift_col(&self, col: usize) -> usize {
        (col + 1) % (self.dims.x - 1)
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

#[test]
fn test_random_piece() {
    let mut board = Board::new(None);

    board.add_queen(1,1);

    let res = match board.random_piece() {
        Some(_) => true,
        None => false
    };

    assert!(res);
}
