use crate::app::App;
use crate::entities::game_object::GameObject;
use crate::types::color::Color;

use rgx::core::*;
use rgx::kit::shape2d::{Fill, Shape};

/// A rectable 🤷‍♂️
pub struct Rectangle {
    pub game_object: GameObject,

    pub fill_color: Color,
    pub stroke_size: f32,
    pub stroke_color: Color,

    index: usize,
    is_init: bool,
}

impl Copy for Rectangle {}

impl Clone for Rectangle {
    fn clone(&self) -> Rectangle {
        *self
    }
}

impl Rectangle {
    /// Default constructor to initialize rectangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),

            fill_color: Color::white(),
            stroke_size: 0.,
            stroke_color: Color::white(),

            index: 0,
            is_init: false,
        };
    }

    /// Render rectangle on screen
    pub fn draw(&mut self, app: &mut App) {
        if self.is_init {
            app.shapes.remove(self.index);
        } else {
            self.is_init = true;
        }

        // TODO: Implement parallax and bound for object position

        app.shapes.push(
            Shape::rect(
                [
                    self.game_object.transform.position.x 
                        + app.width() as f32 / 2.
                        - self.game_object.transform.size.x / 2.,
                    self.game_object.transform.position.y 
                        + app.height() as f32 / 2.
                        - self.game_object.transform.size.y / 2.,
                ],
                [
                    self.game_object.transform.position.x
                        + app.width() as f32 / 2.
                        + self.game_object.transform.size.x / 2.,
                    self.game_object.transform.position.y
                        + app.height() as f32 / 2.
                        + self.game_object.transform.size.y / 2.,
                ],
            )
            .stroke(
                self.stroke_size,
                Rgba::new(
                    self.stroke_color.r,
                    self.stroke_color.g,
                    self.stroke_color.b,
                    self.stroke_color.a,
                ),
            )
            .fill(Fill::solid(Rgba::new(
                self.fill_color.r,
                self.fill_color.g,
                self.fill_color.b,
                self.fill_color.a,
            ))),
        );
    }
}
