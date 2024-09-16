mod canvas;

mod capture;

mod compositor;
pub use compositor::*;
use wgpu::{Device, Texture};

use crate::macros::driver_wrapper;

mod syscall;

driver_wrapper!(
    ["A type wrapper of [`DriverCompositor`](syscall::DriverCompositor)"]
    WgpuLayer[syscall::DriverWgpuLayer]
);

fn create_layer_texture(device: &Device, width: u32, height: u32) -> Texture {
    let texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: None,
        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
    };

    device.create_texture(&texture_desc)
}

#[cfg(test)]
mod tests {

    use futures::poll;
    use image::{ImageBuffer, Rgb};

    use crate::{Compositor, Result, Vertex};

    use super::*;

    async fn init() -> Result<(Compositor, Texture)> {
        _ = pretty_env_logger::try_init_timed();
        let (compositor, texture) = WgpuCompositor::new().to_texture(256, 256).await?;

        Ok((compositor.into(), texture))
    }

    #[futures_test::test]
    async fn canvas_test() {
        let (compositor, _texture) = init().await.unwrap();

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

        let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, data).unwrap();

        buffer.save("image.png").unwrap();
    }
}
