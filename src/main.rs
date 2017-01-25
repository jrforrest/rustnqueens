#![allow(dead_code)]

extern crate rand;
extern crate cursive;

mod pos;
mod board;
mod solution;
mod solver;
mod ui;

use ui::UI;

fn main() {
    let mut ui = UI::new();
    ui.run();
}
