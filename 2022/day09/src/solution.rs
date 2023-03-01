mod knots;
mod movements;
mod simulations;

use std::io::BufRead;

use crate::configuration::Configuration;

use self::{movements::movement::Movement, simulations::rope_simulation::RopeSimulation};

pub fn run(config: Configuration) -> Result<String, String> {
    if config.input_file_buffer.is_some() {
        let mut bridge = RopeSimulation::new_with_knots(10);

        for line in config.input_file_buffer.unwrap().lines() {
            if let Ok(s) = line {
                bridge.move_rope(Movement::from_string(s));
            }
        }

        //dbg!(&bridge.positions_tail_visited);

        Ok(bridge.positions_tail_visited.len().to_string())
    } else {
        panic!("No file provided!");
    }
}
