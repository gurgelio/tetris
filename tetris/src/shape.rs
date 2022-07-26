use wasm_bindgen::prelude::*;

use crate::direction::Direction;

use crate::position::Position;
use std::collections::HashSet;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Position>,
    global_position: Position,
    color: u8,
}

impl Shape {
    fn new(positions: [Position; 4], global_position: Position, color: u8) -> Self {
        Self {
            positions: positions.into_iter().collect(),
            global_position,
            color,
        }
    }

    pub fn remove_line(&mut self, y: isize) {
        self.positions = self
            .positions
            .iter()
            .filter(|pos| pos.1 - self.global_position.1 != y)
            .copied()
            .map(|pos| {
                if pos.1 >= y {
                    pos
                } else {
                    Position(pos.0, pos.1 + 1)
                }
            })
            .collect();
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.positions.iter().map(|&pos| pos + self.global_position)
    }

    pub fn fall(mut self) -> Self {
        self.global_position.1 += 1;
        self
    }

    pub fn shift(mut self, direction: Direction) -> Self {
        match direction {
            Direction::Left => self.global_position.0 += 1,
            Direction::Right => self.global_position.0 -= 1,
        }
        self
    }

    pub fn rotate(self, direction: Direction) -> Self {
        self.positions.iter().for_each(|pos| pos.rotate(&direction));
        self
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.positions
            .intersection(&other.positions().collect())
            .count()
            > 0
    }

    pub fn new_i(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(-1, 0),
                Position(0, 0),
                Position(1, 0),
                Position(2, 0),
            ],
            global_position,
            color,
        )
    }

    pub fn new_o(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(0, 0),
                Position(1, 0),
                Position(0, 1),
                Position(1, 1),
            ],
            global_position,
            color,
        )
    }

    pub fn new_t(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(-1, 0),
                Position(0, 0),
                Position(1, 0),
                Position(0, 1),
            ],
            global_position,
            color,
        )
    }

    pub fn new_j(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(0, -1),
                Position(0, 0),
                Position(0, 1),
                Position(-1, 1),
            ],
            global_position,
            color,
        )
    }

    pub fn new_l(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(0, -1),
                Position(0, 0),
                Position(0, 1),
                Position(1, 1),
            ],
            global_position,
            color,
        )
    }

    pub fn new_s(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(0, 0),
                Position(1, 0),
                Position(0, 1),
                Position(-1, 1),
            ],
            global_position,
            color,
        )
    }

    pub fn new_z(global_position: Position, color: u8) -> Self {
        Self::new(
            [
                Position(0, 0),
                Position(-1, 0),
                Position(0, 1),
                Position(1, 1),
            ],
            global_position,
            color,
        )
    }
}
