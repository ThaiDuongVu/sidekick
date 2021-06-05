use crate::entities::game_object::GameObject;
use crate::types::color::Color;

pub struct Triangle {
    game_object: GameObject,
    color: Color,
}

impl Triangle {
    /// Default constructor to initialize triangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::white(),
        };
    }
}
