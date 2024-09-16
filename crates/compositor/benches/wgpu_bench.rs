use std::time::Instant;

use hala_graphics_compositor::{syscall::DriverCompositor, Vertex, WgpuCompositor};

fn main() {
    futures::executor::block_on(wgpu_bench());
}

async fn wgpu_bench() {
    let compositor = WgpuCompositor::new().create(1920, 1080).await.unwrap();

    let canvas = compositor.create_canvas(None).await.unwrap();

    const VERTICES: &[Vertex] = &[
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // B
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // E
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // C
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // E
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            color: [0.5, 0.0, 0.5],
        }, // E
    ];

    const INDICES: &[u32] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

    println!("\n\nwgpu_compositor:");

    let timer = Instant::now();

    for _ in 0..1000 {
        canvas
            .update(VERTICES.to_vec(), INDICES.to_vec())
            .await
            .unwrap();

        compositor.compositing().await.unwrap();
    }

    println!("\t frame rate: {:?}", timer.elapsed() / 1000)
}
