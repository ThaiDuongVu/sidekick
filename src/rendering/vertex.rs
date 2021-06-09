#[derive(Default, Debug, Clone)]
pub struct Vertex {
    // X and Y coordinates of vertex
    pub positions: [f32; 2],
    // RGBA color values of vertex
    pub color: [f32; 4],
}
