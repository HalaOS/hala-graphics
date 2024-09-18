use wgpu_derive::Vertex;

#[derive(Vertex, Default)]
pub struct Mock {
    #[skip]
    pub message: String,

    pub u8x2: [u8; 2],
    // invalid format.
    // pub u8x3: [u8; 3],
    pub u8x4: [u8; 4],
    #[normalize]
    pub unorm8x2: [u8; 2],
    #[normalize]
    pub unorm8x4: [i8; 4],
    pub i8x2: [i8; 2],
    // invalid format.
    // pub i8x3: [i8; 3],
    pub i8x4: [i8; 4],
    #[normalize]
    pub snorm8x2: [i8; 2],
    #[normalize]
    pub snorm8x4: [i8; 4],

    pub u16x2: [u16; 2],
    /// invalid format.
    // pub u8x3: [u8; 3],
    pub u16x4: [u16; 4],
    #[normalize]
    pub unorm16x2: [u16; 2],
    #[normalize]
    pub unorm16x4: [u16; 4],
    pub i16x2: [i16; 2],
    // invalid format.
    // pub i8x3: [i8; 3],
    pub i16x4: [i16; 4],
    #[normalize]
    pub snorm16x2: [i16; 2],
    #[normalize]
    pub snorm16x4: [i16; 4],

    pub f32x2: [f32; 2],
    pub f32x3: [f32; 3],
    pub f32x4: [f32; 4],

    pub u32x2: [u32; 2],
    pub u32x3: [u32; 3],
    pub u32x4: [u32; 4],

    pub i32x2: [i32; 2],
    pub i32x3: [i32; 3],
    pub i32x4: [i32; 4],
    // invalid format.
    // pub i32x6: [i32; 6],
}

#[test]
fn get_buffer_layout() {
    let vertex = Mock::default();

    let mut buf = vec![0u8; 1024];

    vertex.to_bytes(&mut buf);
}
