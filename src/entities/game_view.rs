use crate::entities::game_object::GameObject;
use crate::types::color::Color;

/// Current game viewport
pub struct GameView {
    pub game_object: GameObject,
    pub color: Color,
}

impl GameView {
    /// Default constructor to initialize viewport
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::black(),
        };
    }
}
