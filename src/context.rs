use crate::winit::event_loop::ControlFlow;
use std::rc::Rc;

/// A reference to an instance of this struct is passed to most methods on
/// Game. This struct allows Game methods to be able to interact
/// with yagl about App related things (e.g. requesting to exit)
pub struct AppContext<'a> {
    #[allow(dead_code)]
    pub(crate) globals: &'a Rc<Globals>,
    pub(crate) control_flow: &'a mut ControlFlow,
}

impl<'a> AppContext<'a> {
    pub fn exit(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
}

/// Various global values needed by various parts of the system
pub(crate) struct Globals {
}

impl Globals {
    pub fn new() -> Rc<Globals> {
        Rc::new(Globals {})
    }
}
