use crate::components::transform::Transform;
use crate::types::color::Color;

/// Current game viewport
pub struct GameView {
    pub transform: Transform,
    pub color: Color,
}

impl GameView {
    /// Default constructor to initialize viewport
    pub fn new() -> Self {
        return Self {
            transform: Transform::new(),
            color: Color::black(),
        };
    }
}
