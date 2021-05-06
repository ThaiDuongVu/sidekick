use std::fmt;

pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for Vector2 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Copy for Vector2 {}

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        *self
    }
}

impl Vector2 {
    // Default constructor to initialize Vector2
    pub fn new() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }

    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }
    pub fn identity() -> Self {
        return Self { x: 1.0, y: 1.0 };
    }
    pub fn unit_x() -> Self {
        return Self { x: 1.0, y: 0.0 };
    }
    pub fn unit_y() -> Self {
        return Self { x: 0.0, y: 1.0 };
    }
}
