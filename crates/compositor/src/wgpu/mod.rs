mod layers;
mod rendering;

mod capture;

mod compositor;
pub use compositor::*;

use crate::macros::driver_wrapper;

mod syscall;
pub use syscall::*;

driver_wrapper!(
    ["A type wrapper of [`WgpuLayer`](syscall::DriverWgpuLayer)"]
    WgpuLayer[syscall::DriverWgpuLayer]
);

driver_wrapper!(
    ["A type wrapper of [`WgpuLayerRender`](syscall::DriverWgpuLayerRender)"]
    WgpuRendering[syscall::DriverWgpuRendering]
);

#[cfg(test)]
mod tests {

    use std::io::Write;

    use futures::poll;

    use crate::{Compositor, Result, Vertex};

    use super::*;

    async fn init() -> Result<Compositor> {
        _ = pretty_env_logger::try_init_timed();
        let compositor = WgpuCompositor::new().create(1920, 1080).await?;

        Ok(compositor.into())
    }

    pub fn save_image(image_data: Vec<u8>, texture_dims: (u32, u32), path: &str) {
        let mut png_data = Vec::<u8>::with_capacity(image_data.len());
        let mut encoder = png::Encoder::new(
            std::io::Cursor::new(&mut png_data),
            texture_dims.0 as u32,
            texture_dims.1 as u32,
        );
        encoder.set_color(png::ColorType::Rgba);
        let mut png_writer = encoder.write_header().unwrap();
        png_writer.write_image_data(&image_data[..]).unwrap();
        png_writer.finish().unwrap();
        log::info!("PNG file encoded in memory.");

        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(&png_data[..]).unwrap();
        log::info!("PNG file written to disc as \"{}\".", path);
    }

    #[futures_test::test]
    async fn canvas_test() {
        let compositor = init().await.unwrap();

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

        canvas
            .update(VERTICES.to_vec(), INDICES.to_vec())
            .await
            .unwrap();

        let mut capture = canvas.capture();

        assert!(poll!(&mut capture).is_pending());

        compositor.compositing().await.unwrap();

        let data = capture.await.unwrap();

        let (width, height) = compositor.size().await.unwrap();

        log::trace!("image size({},{})", width, height);

        save_image(data, (width, height), "image.png");
    }
}
