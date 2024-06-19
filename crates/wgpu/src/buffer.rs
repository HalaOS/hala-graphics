/// Vertex object for copying data from the vertex buffer or copying data to the vertex buffer.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
