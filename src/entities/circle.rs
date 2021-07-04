use crate::app::App;
use crate::entities::game_object::GameObject;
use crate::entities::game_object::RenderLayer;
use crate::types::color::Color;

use rgx::core::*;
use rgx::kit::shape2d::{Fill, Shape};
use rgx::kit::ZDepth;
use rgx::math::Point2;

/// A circle 🤷‍♂️
pub struct Circle {
    pub game_object: GameObject,

    pub fill_color: Color,
    pub stroke_size: f32,
    pub stroke_color: Color,
    pub layer: RenderLayer,

    index: usize,
    is_init: bool,
}

impl Copy for Circle {}

impl Clone for Circle {
    fn clone(&self) -> Circle {
        *self
    }
}

impl Circle {
    /// Default constructor to initialize circle
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

    /// Render circle on screen
    pub fn draw(&mut self, app: &mut App) {
        if self.is_init {
            app.shapes.remove(self.index);
        } else {
            self.is_init = true;
        }

        let x = self.game_object.transform.position.x;
        let y = self.game_object.transform.position.y;

        let radius = self.game_object.transform.radius;

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
            let max_x = app.width() as f32 / 2. - radius + viewport_x / 2.;
            let min_x = -(app.width() as f32) / 2. + radius + viewport_x / 2.;
            if x > max_x {
                self.game_object.transform.position.x = max_x;
            } else if x < min_x {
                self.game_object.transform.position.x = min_x;
            }

            let max_y = app.height() as f32 / 2. - radius + viewport_y / 2.;
            let min_y = -(app.height() as f32) / 2. + radius + viewport_y / 2.;
            if y > max_y {
                self.game_object.transform.position.y = max_y;
            } else if y < min_y {
                self.game_object.transform.position.y = min_y;
            }
        }

        app.shapes.push(
            Shape::circle(
                Point2::new(
                    x + app.width() as f32 / 2. - viewport_x / 2.,
                    y + app.height() as f32 / 2. - viewport_y / 2.,
                ),
                self.game_object.transform.radius,
                32,
            )
            .fill(Fill::solid(Rgba::new(
                self.fill_color.r,
                self.fill_color.g,
                self.fill_color.b,
                self.fill_color.a,
            )))
            .stroke(
                self.stroke_size,
                Rgba::new(
                    self.stroke_color.r,
                    self.stroke_color.g,
                    self.stroke_color.b,
                    self.stroke_color.a,
                ),
            )
            .zdepth(ZDepth::from(self.layer as i32 as f32 / 10.)),
        );

        self.index = app.shapes.len() - 1;
    }
}
