use std::ops::{Add, AddAssign};

use wasm_bindgen::prelude::*;

use crate::direction::Direction;

#[wasm_bindgen]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position(pub isize, pub isize);

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Position {
    pub fn rotate(mut self, direction: &Direction) {
        let tmp = self.0;
        match direction {
            Direction::Left => {
                self.0 = -self.1;
                self.1 = tmp;
            }
            Direction::Right => {
                self.0 = self.1;
                self.1 = -tmp;
            }
        }
        self.0 = -self.1;
        self.1 = tmp;
    }
}
