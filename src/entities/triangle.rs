use crate::entities::game_object::GameObject;
use crate::types::color::Color;

/// A triangle 🤷‍♂️
pub struct Triangle {
    pub game_object: GameObject,
    pub color: Color,
}

impl Copy for Triangle {}

impl Clone for Triangle {
    fn clone(&self) -> Triangle {
        *self
    }
}

impl Triangle {
    /// Default constructor to initialize triangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::white(),
        };
    }

    /// Render triangle on screen
    pub fn draw(&mut self) {}
}
