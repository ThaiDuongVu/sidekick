use crate::components::transform::Transform;
use crate::types::vector2::Vector2;

pub enum DrawMode {
    Fill,
    Line,
}

/// An game object is an entity that exist in the game world
pub struct GameObject {
    pub transform: Transform,
    pub is_visible: bool,
    pub is_parallax: bool,
    pub is_bounded: bool,
}

impl GameObject {
    /// Default constructor to initialize game object
    pub fn new() -> Self {
        return Self {
            transform: Transform::new(),
            is_visible: true,
            is_parallax: false,
            is_bounded: false,
        };
    }

    /// Move object at movement vector
    pub fn r#move(&mut self, movement: Vector2) {
        self.transform.position.translate(movement);
    }

    /// Rotate object at rotation speed
    pub fn rotate(&mut self, rotation: f32) {
        self.transform.rotation += rotation;

        // Clamp rotation to be between 0 and 360
        if self.transform.rotation >= 360.0 {
            self.transform.rotation = 0.0;
        }
    }

    /// Scale object width and height
    pub fn scale(&mut self, scale: Vector2) {
        self.transform.size += scale;
    }

    /// Destroy current game object
    pub fn destroy(&mut self) {
        // TODO: destroy object
    }
}
