use cursive::Cursive;
use cursive::views::{TextView, IdView, LinearLayout, BoxView, Panel};
use cursive::view::SizeConstraint;
use cursive::align::VAlign;

use cursive::direction::Orientation;

use std::cell::RefCell;
use std::rc::Rc;

use solver::Solver;

pub struct UI {
    solver: Rc<RefCell<Solver>>,
    curs: Cursive,
}

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
        let tv = TextView::new("grid").v_align(VAlign::Top);
        let trv = IdView::new("grid", tv);

        let mut lv = LinearLayout::new(Orientation::Vertical);

        lv.add_child(trv);
        lv.add_child(UI::make_lower_box());

        self.curs.add_layer(lv);
    }

    fn make_lower_box() -> BoxView<Panel<IdView<TextView>>> {
        let tv = TextView::new("label").v_align(VAlign::Bottom);
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

        self.set_textview("grid", format!("{}", board));
        self.set_textview("label", format!(" {:?}", board.get_queens()));
    }

    fn set_textview(&mut self, id: &str, text: String) {
        let view = self.curs.find_id::<TextView>(id);
        match view {
            None => panic!("Could not find UI elem with id: {}", id),
            Some(v) => v.set_content(text),
        };
    }
}
