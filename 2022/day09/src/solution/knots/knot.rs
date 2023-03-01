use std::{cell::RefCell, rc::Rc};

use super::knot_position::KnotPosition;

pub type KnotLink = Rc<RefCell<Knot>>;

#[derive(Debug, Clone)]
pub struct Knot {
    pub number: usize,
    pub position: KnotPosition,
    pub next: Option<KnotLink>,
}

impl Eq for Knot {}

impl PartialEq for Knot {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Knot {
    pub fn new(number: usize) -> KnotLink {
        let new_knot = Self {
            number: number,
            position: KnotPosition::new(),
            next: None,
        };

        Rc::new(RefCell::new(new_knot))
    }

    pub fn print_knot(&self) {
        println!("Knot {} - {}", self.number, self.position.to_string());
    }
}
