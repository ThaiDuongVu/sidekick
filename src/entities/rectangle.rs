use crate::app::App;
use crate::entities::game_object::GameObject;
use crate::entities::game_object::RenderLayer;
use crate::types::color::Color;
use crate::types::vector2::Vector2;

use rgx::core::*;
use rgx::kit::shape2d::{Fill, Shape};
use rgx::kit::ZDepth;
use rgx::math::Point2;

/// A rectable 🤷‍♂️
pub struct Rectangle {
    pub game_object: GameObject,

    pub fill_color: Color,
    pub stroke_size: f32,
    pub stroke_color: Color,
    pub layer: RenderLayer,

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
            layer: RenderLayer::Layer1,

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

        let x = self.game_object.transform.position.x;
        let y = self.game_object.transform.position.y;

        let width = self.game_object.transform.size.x;
        let height = self.game_object.transform.size.y;

        let viewport_x = if self.game_object.is_parallax {
            0.
        } else {
            app.game_view.game_object.transform.position.x
        };
        let viewport_y = if self.game_object.is_parallax {
            0.
        } else {
            app.game_view.game_object.transform.position.y
        };

        // Bound object in game view
        if self.game_object.is_bounded {
            let max_x = app.width() as f32 / 2. - width / 2. + viewport_x / 2.;
            let min_x = -(app.width() as f32) / 2. + width / 2. + viewport_x / 2.;
            if x > max_x {
                self.game_object.transform.position.x = max_x;
            } else if x < min_x {
                self.game_object.transform.position.x = min_x;
            }

            let max_y = app.height() as f32 / 2. - height / 2. + viewport_y / 2.;
            let min_y = -(app.height() as f32) / 2. + height / 2. + viewport_y / 2.;
            if y > max_y {
                self.game_object.transform.position.y = max_y;
            } else if y < min_y {
                self.game_object.transform.position.y = min_y;
            }
        }

        // Vertex 0: Top left
        let vertex_0 = Vector2 {
            x: x + app.width() as f32 / 2. - width / 2. - viewport_x / 2.,
            y: y + app.height() as f32 / 2. - height / 2. - viewport_y / 2.,
        };
        // Vertex 1: Bottom right
        let vertex_1 = Vector2 {
            x: x + app.width() as f32 / 2. + width / 2. - viewport_x / 2.,
            y: y + app.height() as f32 / 2. + height / 2. - viewport_y / 2.,
        };

        let center = Vector2 {
            x: (vertex_0.x + vertex_1.x) / 2.,
            y: (vertex_0.y + vertex_1.y) / 2.,
        };

        app.shapes.push(
            Shape::rect([vertex_0.x, vertex_0.y], [vertex_1.x, vertex_1.y])
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
                )))
                .rotation(
                    self.game_object.transform.rotation,
                    Point2::new(center.x, center.y),
                )
                .zdepth(ZDepth::from(self.layer as i32 as f32 / 10.)),
        );
    }
}
