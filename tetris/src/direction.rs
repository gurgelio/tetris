pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("Direção inválida!"),
        }
    }
}
