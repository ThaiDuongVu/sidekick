use crate::types::vector2::Vector2;
use std::fmt;

/// A component that represents the position, rotation and scale of a game object
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub size: Vector2,
}

impl fmt::Display for Transform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Position: {} \nRotation: {} \nScale: {}",
            self.position, self.rotation, self.size
        )
    }
}

impl Copy for Transform {}

impl Clone for Transform {
    fn clone(&self) -> Transform {
        *self
    }
}

impl Transform {
    /// Default constructor to initialize Transform
    pub fn new() -> Self {
        return Self {
            position: Vector2::zero(),
            rotation: 0.0,
            size: Vector2 { x: 50.0, y: 50.0 },
        };
    }
}
