use crate::position::Position;

use crate::shape::Shape;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;

pub struct Bag {
    shapes: [fn(Position, u8) -> Shape; 7],
    colors: [u8; 7],
    shape_weights: [u8; 7],
    rng: ThreadRng,
    remaining: u8,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            shapes: [
                Shape::new_i,
                Shape::new_o,
                Shape::new_t,
                Shape::new_j,
                Shape::new_l,
                Shape::new_s,
                Shape::new_z,
            ],
            colors: [1, 1, 1, 1, 1, 1, 1],
            shape_weights: [1, 1, 1, 1, 1, 1, 1],

            rng: thread_rng(),
            remaining: 7,
        }
    }

    pub fn pick(&mut self, global_position: Position) -> Shape {
        if self.remaining == 0 {
            self.refill();
        }

        self.remaining -= 1;
        self.pick_shape(global_position)
    }

    fn pick_shape(&mut self, global_position: Position) -> Shape {
        let dist = WeightedIndex::new(&self.shape_weights).unwrap();
        let chosen = dist.sample(&mut self.rng);
        self.shape_weights[chosen] = 0;
        self.shapes[chosen](global_position, self.pick_color())
    }

    fn pick_color(&mut self) -> u8 {
        let dist = WeightedIndex::new(&self.color_weights).unwrap();
        let chosen = dist.sample(&mut self.rng);
        self.color_weights[chosen] = 0;
        chosen
    }

    fn refill(&mut self) {
        for i in 0..7 {
            self.shape_weights[i] = 1;
            self.color_weights[i] = 1;
        }
        self.remaining = 7;
    }
}
