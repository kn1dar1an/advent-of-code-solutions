use std::collections::HashMap;

use crate::solution::{
    knots::{
        knot::{Knot, KnotLink},
        knot_position::KnotPosition,
    },
    movements::{direction::Direction, movement::Movement},
};

#[derive(Debug, PartialEq, Eq)]
pub struct RopeSimulation {
    pub n_knots: usize,
    pub head: KnotLink,
    pub tail: KnotLink,
    pub positions_tail_visited: HashMap<String, usize>,
}

impl RopeSimulation {
    pub fn new_with_knots(n_knots: usize) -> Self {
        let head: KnotLink = Knot::new(0);
        let mut tail: Option<KnotLink> = None;

        let mut temp_current_knot: KnotLink = head.clone();
        for n in 1..n_knots {
            let next_knot_link: KnotLink = Knot::new(n);

            temp_current_knot.borrow_mut().next = Some(next_knot_link.clone());

            if n == n_knots - 1 {
                tail = Some(next_knot_link);
            } else {
                temp_current_knot = next_knot_link;
            };
        }

        let mut simulation = Self {
            n_knots,
            head,
            tail: tail.expect("Expected a tail"),
            positions_tail_visited: HashMap::new(),
        };

        simulation
            .positions_tail_visited
            .insert(KnotPosition::new().to_string(), 1);

        simulation
    }

    pub fn mark_tail_position(&mut self) {
        let key = self.tail.borrow().position.to_string();
        if let Some(count) = self.positions_tail_visited.get_mut(&key) {
            *count += 1;
        } else {
            self.positions_tail_visited.insert(key, 1);
        }
    }

    pub fn move_rope(&mut self, movement: Movement) {
        // Go one step at a time, avoid moving subsequent knots if not needed.
        let movement_unit: i32 = movement.amount / movement.amount.abs();

        for _ in 0..movement.amount.abs() {
            {
                let mut head = self.head.borrow_mut();

                match movement.direction {
                    Direction::Horizontal => head.position.x_pos += movement_unit,
                    Direction::Vertical => head.position.y_pos += movement_unit,
                }
            }
            self.affect_next_knot(&self.head);
            self.mark_tail_position();
        }
    }

    fn affect_next_knot(&self, current_link: &KnotLink) {
        let current = current_link.borrow();

        let should_affect_tail: bool = if let Some(next_link) = &current.next {
            let mut next = next_link.borrow_mut();
            let (dx, dy) = current.position.distance_to(&next.position);
            if dx.abs() > 1 || dy.abs() > 1 {
                next.position.x_pos += if dx == 0 { 0 } else { dx / dx.abs() };
                next.position.y_pos += if dy == 0 { 0 } else { dy / dy.abs() };
                true
            } else {
                false
            }
        } else {
            false
        };

        if let Some(next_link) = &current.next {
            if should_affect_tail {
                self.affect_next_knot(&next_link);
            }
        };
    }
}
