use cursive::Cursive;
use cursive::views::{TextView, IdView, LinearLayout, BoxView, Panel};
use cursive::view::SizeConstraint;
use cursive::align::VAlign;

use cursive::direction::Orientation;

use std::cell::RefCell;
use std::rc::Rc;

use solver::Solver;
use board::Board;
use pos::Pos;

use std::string::String;

pub struct UI {
    solver: Rc<RefCell<Solver>>,
    curs: Cursive,
}

const GRID_MSG: &'static str = "I got N problems and Queens are all of em'";
const STATUS_MSG: &'static str = "s: iterate\tq: quit";

impl UI {
    pub fn new() -> UI {
        let curs = Cursive::new();
        let solver = Rc::new(RefCell::new(Solver::new()));

        UI{solver: solver, curs: curs}
    }

    pub fn run(&mut self) {
        self.init_views();
        self.init_callbacks();

        self.curs.run();
    }

    fn init_views(&mut self) {
        let tv = TextView::new(GRID_MSG).v_align(VAlign::Top);
        let trv = IdView::new("grid", tv);

        let mut lv = LinearLayout::new(Orientation::Vertical);

        lv.add_child(trv);
        lv.add_child(UI::make_lower_box());

        self.curs.add_layer(lv);
    }

    fn make_lower_box() -> BoxView<Panel<IdView<TextView>>> {
        let tv = TextView::new(STATUS_MSG).v_align(VAlign::Bottom);
        let trv = IdView::new("label", tv);
        let panel = Panel::new(trv);
        let bv = BoxView::new(SizeConstraint::Full, SizeConstraint::Fixed(4), panel);

        bv
    }

    fn init_callbacks(&mut self) {
        self.curs.add_global_callback('q', |u| u.quit());
        let solver_cell = self.solver.clone();

        self.curs.add_global_callback('s', move |u| {
            solver_cell.borrow_mut().step();
            let srf = &solver_cell.borrow();
            UiBinding::new(u, srf).update()
        });
    }
}

/// Binds Solver props to the UI
struct UiBinding<'a> {
    curs: &'a mut Cursive,
    solver: &'a Solver,
}

impl<'a> UiBinding<'a> {
    fn new(curs: &'a mut Cursive, solver: &'a Solver) -> UiBinding<'a> {
        UiBinding{curs: curs, solver: solver}
    }

    pub fn update(&mut self) {
        let board = self.solver.get_board();

        self.set_textview("grid", BoardUI(board).grid());
        self.set_textview("label", UiBinding::status_line(self.solver));
    }

    fn status_line(solver: &Solver) -> String {
        format!("Iteration: {}\tJep Queens: {}\tTemp: {}\nBest Jep Queens: {}",
            solver.iteration,
            solver.n_jeporadized_queens(),
            solver.temp,
            solver.best_n_jeporadized_queens())
    }

    fn set_textview(&mut self, id: &str, text: String) {
        let view = self.curs.find_id::<TextView>(id);
        match view {
            None => panic!("Could not find UI elem with id: {}", id),
            Some(v) => v.set_content(text),
        };
    }
}

struct BoardUI<'a> (&'a Board);

impl<'a> BoardUI<'a> {
    pub fn grid(&self) -> String {
        let mut out = String::new();

        for y in 0..(self.0.dims.y - 1) {
            for x in 0..(self.0.dims.x - 1) {
                let ch = if self.0.queen_at(Pos{x: x, y: y}) {
                    'x'
                } else {
                    '~'
                };
                out.push(ch)
            }
            out.push('\n');
        }

        out
    }
}
