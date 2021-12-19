use crate::model::Model;
use crate::terminalview::TerminalView;
use std::cell::RefCell;
use std::rc::Rc;


trait Controller {
    fn process_input() -> ();
}


pub struct TerminalController<'a> {
    model: Rc<RefCell<Model>>,
    view: &'a TerminalView,
}


impl<'a> TerminalController<'a> {
    pub fn new(model: Rc<RefCell<Model>>, view: &'a TerminalView) -> TerminalController<'a> {
        TerminalController {
            model: model,
            view: view,
        }
    }
}


impl<'a> Controller for TerminalController<'a> {
    fn process_input() -> () {

    }
}

