use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {
    pub fn new() -> Hall {
        Hall {
            left: None,
            right: None,
        }
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
        unimplemented!();
    }
}
