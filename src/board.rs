use pos::*;
use std::collections::HashSet;
use std::option::Option;

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

    //pub fn populate(&self) -> Vec<&pos> {
    //}
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

fn test_default_size() {
    let board = Board::new(None);
    assert!(board.dims.x == 10);
    assert!(board.dims.y == 10);
}
