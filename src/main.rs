extern crate rand;
extern crate cursive;

mod pos;
mod board;
mod solution;
mod solver;

use solver::Solver;

use cursive::Cursive;
use cursive::views::{TextView, IdView};

use std::fmt;

use std::cell::RefCell;

fn main() {
    let mut ui = Cursive::new();

    ui.add_global_callback('q', |u| u.quit());

    let cell = RefCell::new(Solver::new());

    let tv = TextView::new("Hello!");
    let trv = IdView::new("text", tv);
    ui.add_layer(trv);

    ui.add_global_callback('s', move |u| {
        let id = "text";
        let view = u.find_id::<TextView>(id);

        cell.borrow_mut().step();

        match view {
            None => panic!("Nothing found with id: {}", id),
            Some(v) => v.set_content(format!("{}", cell.borrow().get_board())),
        };
    });

    ui.run();

   //# for _ in 0..10 {
   //#     println!("{}", solver.get_board());
   //#     solver.step();
   //# }
}
