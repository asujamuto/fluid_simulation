use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub(crate) position: [f32; 2],
}
implement_vertex!(Vertex, position);