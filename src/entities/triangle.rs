use crate::app::App;
use crate::entities::game_object::GameObject;
use crate::rendering::vertex::Vertex;
use crate::types::color::Color;
use crate::types::vector2::Vector2;

use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

/// A triangle that can be rendered on screen
pub struct Triangle {
    pub game_object: GameObject,
    pub color: Color,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
    index: usize,
}

impl Triangle {
    // Default constructor to initialize triangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::white(),

            vertex_buffer: None,
            index: 0,
        };
    }

    /// Update triangle vertices
    fn update(&mut self, app: &mut App) {
        // Current triangle position
        let x: f32 = self.game_object.transform.position.x;
        let y: f32 = self.game_object.transform.position.y;

        // Current triangle scale
        let width: f32 = self.game_object.transform.size.x;
        let height: f32 = self.game_object.transform.size.y;

        // Current game view position
        let view_x: f32 = if self.game_object.is_parallax {
            0.0
        } else {
            -app.game_view.game_object.transform.position.x
        };
        let view_y: f32 = if self.game_object.is_parallax {
            0.0
        } else {
            -app.game_view.game_object.transform.position.y
        };
        // Left vertex
        let vertex_0: Vector2 = self.calculate_rotation(
            app,
            Vector2 {
                x: x / (app.width() as f32 / 2.0) - width / app.width() as f32 + view_x,
                y: y / (app.height() as f32 / 2.0) + height / app.height() as f32 + view_y,
            },
        );
        // Right vertex
        let vertex_1: Vector2 = self.calculate_rotation(
            app,
            Vector2 {
                x: x / (app.width() as f32 / 2.0) + width / app.width() as f32 + view_x,
                y: y / (app.height() as f32 / 2.0) + height / app.height() as f32 + view_y,
            },
        );
        // Up vertex
        let vertex_2: Vector2 = self.calculate_rotation(
            app,
            Vector2 {
                x: x / (app.width() as f32 / 2.0) + view_x,
                y: y / (app.height() as f32 / 2.0) - height / app.height() as f32 + view_y,
            },
        );

        // vertex_0 = self.calculate_rotation(app, vertex_0);

        // Implement vertex buffer to store triangle points
        self.vertex_buffer = Some({
            vulkano::impl_vertex!(Vertex, positions, color);
            CpuAccessibleBuffer::from_iter(
                app.device.as_ref().unwrap().clone(),
                BufferUsage::all(),
                false,
                [
                    Vertex {
                        positions: [vertex_0.x, vertex_0.y],
                        color: [0.0, 0.0, 1.0, 1.0],
                    },
                    Vertex {
                        positions: [vertex_1.x, vertex_1.y],
                        color: [0.0, 0.0, 1.0, 1.0],
                    },
                    Vertex {
                        positions: [vertex_2.x, vertex_2.y],
                        color: [0.0, 0.0, 1.0, 1.0],
                    },
                ]
                .iter()
                .cloned(),
            )
            .unwrap()
        });
    }

    /// Calculate vertex's position based on current rotation
    fn calculate_rotation(&mut self, app: &mut App, vertex: Vector2) -> Vector2 {
        // Sine & Cosine of current rotation
        let sin: f32 = self.game_object.transform.rotation.to_radians().sin();
        let cos: f32 = self.game_object.transform.rotation.to_radians().cos();
        // Point of rotation
        let x_point: f32 = self.game_object.transform.position.x;
        let y_point: f32 = self.game_object.transform.position.y;
        // Current vertex point
        let vertex_x: f32 = vertex.x * (app.width() as f32 / 2.0);
        let vertex_y: f32 = vertex.y * (app.height() as f32 / 2.0);
        // Rotate vertex vector to match with current rotation
        return Vector2 {
            x: x_point,
            y: y_point,
        } + Vector2 {
            x: cos * (vertex_x - x_point) - sin * (vertex_y - y_point),
            y: sin * (vertex_x - x_point) + cos * (vertex_y - y_point),
        } / Vector2 {
            x: app.width() as f32 / 2.0,
            y: app.height() as f32 / 2.0,
        };
    }
    /// Render triangle on screen
    pub fn draw(&mut self, app: &mut App) {
        // Do not render anything if object is not visible
        if !self.game_object.is_visible {
            return;
        }

        self.update(app);

        // Add current vertex buffer to app's vector of vertex buffers
        if app.vertex_buffers.len() > 0 {
            app.vertex_buffers.remove(self.index);
        }
        app.vertex_buffers
            .push(self.vertex_buffer.as_ref().unwrap().clone());
        // Reset index of current vertex buffer in vector
        self.index = app.vertex_buffers.len() - 1;
    }
}
