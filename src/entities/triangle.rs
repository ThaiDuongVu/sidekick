use gl;
use crate::game_object::GameObject;
use crate::types::color::Color;

/// A triangle that can be rendered on screen
pub struct Triangle {
    pub game_object: GameObject,
    pub color: Color,
}

impl Triangle {
    // Default constructor to initialize triangle
}