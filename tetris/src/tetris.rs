use wasm_bindgen::prelude::*;

use std::collections::HashSet;
use std::mem;

use crate::{bag::Bag, position::Position};

use crate::direction::Direction;
use crate::shape::Shape;

#[wasm_bindgen]
pub struct Tetris {
    width: isize,
    height: isize,
    current: Shape,
    bag: Bag,
    fixed_shapes: Vec<Shape>,
    lost: bool,
}

#[wasm_bindgen]
impl Tetris {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let mut bag = Bag::new();
        Self {
            width: width as isize,
            height: height as isize,
            current: bag.pick(Position(width as isize / 2, 0)),
            bag,
            fixed_shapes: vec![],
            lost: false,
        }
    }

    pub fn get_state(&self) -> Vec<Shape> {
        self.fixed_shapes.clone().push(self.current.clone())
    }

    pub fn tick(&mut self) {
        if self.lost {
            return;
        }

        let falling_shape = self.current.clone().fall();

        if self.is_out_of_bounds(&falling_shape) || self.is_colliding(&falling_shape) {
            let new_fixed_shape = mem::replace(
                &mut self.current,
                self.bag.pick(Position(self.width / 2, 0)),
            );

            self.lost = self.is_colliding(&new_fixed_shape);
            self.fixed_shapes.push(new_fixed_shape);
            self.remove_full_lines();
            return;
        }

        self.current = falling_shape;
    }

    pub fn rotate(&mut self, direction: u32) {
        if self.lost {
            return;
        }

        let rotating_shape = self.current.clone().rotate(Direction::from_u32(direction));

        if self.is_out_of_bounds(&rotating_shape)
            || self.reached_the_ground(&rotating_shape)
            || self.is_colliding(&rotating_shape)
        {
            return;
        }

        self.current = rotating_shape;
    }

    pub fn shift(&mut self, direction: u32) {
        if self.lost {
            return;
        }

        let shifting_shape = self.current.clone().shift(Direction::from_u32(direction));

        if self.is_out_of_bounds(&shifting_shape) || self.is_colliding(&shifting_shape) {
            return;
        }

        self.current = shifting_shape;
    }

    fn remove_full_lines(&mut self) {
        let full_lines: Vec<isize> = (0..self.width)
            .filter(|line| self.is_line_full(*line))
            .collect();

        full_lines.iter().for_each(|line| self.remove_line(*line))
    }

    fn remove_line(&mut self, y: isize) {
        self.fixed_shapes
            .iter_mut()
            .for_each(|shape| shape.remove_line(y));
    }

    fn is_out_of_bounds(&self, shape: &Shape) -> bool {
        shape
            .positions()
            .any(|pos| 0 > pos.0 || pos.0 >= self.width)
    }

    fn reached_the_ground(&self, shape: &Shape) -> bool {
        shape
            .positions()
            .any(|pos| 0 > pos.1 || pos.1 >= self.height)
    }

    fn is_line_full(&self, y: isize) -> bool {
        self.fixed_shapes
            .iter()
            .flat_map(Shape::positions)
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as isize
            == self.width
    }

    fn is_colliding(&self, shape: &Shape) -> bool {
        self.fixed_shapes
            .iter()
            .any(|fixed_shape| fixed_shape.collides_with(shape))
    }
}
