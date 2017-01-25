#[derive(Eq, PartialEq, Clone, Debug)]

pub struct Pos {
    pub x: i32,
    pub y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos {x: x, y: y}
    }

    pub fn can_attack(&self, other: &Pos) -> bool {
        if self == other {
            return false;
        }

        self.on_row_with(other) ||
            self.on_col_with(other) ||
            self.diagonal_to(other)
    }

    fn diagonal_to(&self, other: &Pos) -> bool {
        (other.x - self.x).abs() == (other.y - self.y).abs()
    }

    fn on_row_with(&self, other: &Pos) -> bool {
        other.y == self.y
    }

    fn on_col_with(&self, other: &Pos) -> bool {
        other.x == self.x
    }
}

#[test]
fn test_diagonal() {
    let first = Pos{x: 5, y: 5};
    let second = Pos{x: 6, y: 6};

    assert!(first.diagonal_to(&second));
}

#[test]
fn test_on_row_with() {
    let first = Pos{x: 1, y: 2};
    let second = Pos{x: 3, y: 2};

    assert!(first.on_row_with(&second));
}

#[test]
fn test_equal() {
    let first = Pos{x: 5, y: 5};
    let second = Pos{x: 5, y: 5};

    assert!(first == second);
}

#[test]
fn test_can_attack() {
    let first = Pos{x: 2, y: 1};
    let second = Pos{x: 3, y: 2};
    let third = Pos{x: 3, y: 3};

    assert!(first.can_attack(&second));
    assert!(!first.can_attack(&third));
    assert!(second.can_attack(&third));
}
