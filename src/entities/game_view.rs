use crate::entities::game_object::GameObject;
use crate::types::color::Color;
use crate::types::vector2::Vector2;

use rand::Rng;

/// Current game viewport
pub struct GameView {
    pub game_object: GameObject,
    pub color: Color,

    pub is_shaking: bool,
    shake_intensity: f32,
    shake_duration: f32,
    shake_decrease_factor: f32,
    original_position: Vector2,
}

impl GameView {
    /// Default constructor to initialize viewport
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::black(),
            is_shaking: false,
            shake_intensity: 0.,
            shake_duration: 0.,
            shake_decrease_factor: 0.,
            original_position: Vector2::zero(),
        };
    }

    /// Start shaking the current game view
    pub fn start_shaking(&mut self, intensity: f32, duration: f32, decrease_factor: f32) {
        self.shake_intensity = intensity;
        self.shake_duration = duration;
        self.shake_decrease_factor = decrease_factor;

        self.is_shaking = true;
        self.original_position = self.game_object.transform.position;
    }

    /// Stop shaking the current game view
    pub fn stop_shaking(&mut self) {
        self.is_shaking = false;
        self.game_object.transform.position = self.original_position;
        self.original_position = Vector2::zero();
    }

    /// Randomize viewport position during shake period.
    /// Note: Shaking only affects non-parallax game objects.
    pub fn shake(&mut self, frame_time: f32) {
        // Decrease shake duration every frame
        self.shake_duration -= frame_time * self.shake_decrease_factor;

        // If shake duration runs out then stop shaking
        if self.shake_duration <= 0. {
            self.stop_shaking();
        }

        // Randomize viewport position in intensity range
        self.game_object.transform.position = self.original_position
            + Vector2 {
                x: rand::thread_rng().gen_range(-self.shake_intensity..self.shake_intensity),
                y: rand::thread_rng().gen_range(-self.shake_intensity..self.shake_intensity),
            };
    }
}
