use crate::app::App;
use crate::entities::game_object::GameObject;
use crate::rendering::vertex::Vertex;
use crate::types::color::Color;

use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

/// A triangle that can be rendered on screen
pub struct Triangle {
    pub game_object: GameObject,
    pub color: Color,

    is_init: bool,

    vertex_buffer: Option<Arc<CpuAccessibleBuffer<[Vertex]>>>,
}

impl Triangle {
    // Default constructor to initialize triangle
    pub fn new() -> Self {
        return Self {
            game_object: GameObject::new(),
            color: Color::white(),

            is_init: false,
            vertex_buffer: None,
        };
    }

    /// Initialize triangle
    pub fn init(&mut self, app: &mut App) {
        // Vertex buffer to store triangle points
        self.vertex_buffer = Some({
            vulkano::impl_vertex!(Vertex, positions);
            CpuAccessibleBuffer::from_iter(
                app.device.as_ref().unwrap().clone(),
                BufferUsage::all(),
                false,
                [
                    Vertex {
                        positions: [-0.5, -0.25],
                    },
                    Vertex {
                        positions: [0.0, 0.5],
                    },
                    Vertex {
                        positions: [0.25, -0.1],
                    },
                ]
                .iter()
                .cloned(),
            )
            .unwrap()
        });

        self.is_init = true;
    }

    /// Render triangle on screen
    pub fn draw(&mut self, app: &mut App) {
        if !self.is_init {
            self.init(app);
        }

        if !app
            .vertex_buffers
            .contains(&self.vertex_buffer.as_ref().unwrap().clone())
        {
            app.vertex_buffers
                .push(self.vertex_buffer.as_ref().unwrap().clone());
        }
    }
}
