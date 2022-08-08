use bytemuck::{Pod, Zeroable};

#[derive(Pod, Zeroable, Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
}
