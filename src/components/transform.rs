use std::fmt;
use crate::types::vector2::Vector2;

pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub scale: Vector2,
}

impl fmt::Display for Transform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Position: {} \nRotation: {} \nScale: {}", self.position, self.rotation, self.scale)
    }
}

impl Copy for Transform {}

impl Clone for Transform {
    fn clone(&self) -> Transform {
        *self
    }
}

impl Transform {
    // Default constructor to initialize Transform
    pub fn new() -> Self {
        return Self {
            position: Vector2::zero(),
            rotation: 0.0,
            scale: Vector2::identity(),
        };
    }
}
