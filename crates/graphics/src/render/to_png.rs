use futures::poll;
use wgpu::{CommandEncoderDescriptor, TextureDescriptor};

use crate::{
    render::{render_syscall::DriverElement, CaptureElement},
    wgpu::init_wgpu,
    Png, Result, Viewport,
};

use super::Element;

/// Render an [`Element`] into a png image.
pub async fn render_to_png(
    label: Option<&str>,
    element: &Element,
    viewport: Viewport,
) -> Result<Png<'static>> {
    log::trace!("render_to_file viewport: {:?}", viewport);

    let capture_element = CaptureElement::default();

    let mut capture_fut = Box::pin(capture_element.once());

    assert!(poll!(&mut capture_fut).is_pending());

    let (device, queue) = init_wgpu().await?;

    element.attach(&device);
    capture_element.attach(&device);

    let render_attachment = device.create_texture(&TextureDescriptor {
        size: wgpu::Extent3d {
            width: viewport.width,
            height: viewport.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: None,
        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
    });

    let texture_view = render_attachment.create_view(&Default::default());

    let mut command_encoder = device.create_command_encoder(&CommandEncoderDescriptor { label });

    element.before_redraw(&device, &render_attachment, &mut command_encoder, &viewport);
    capture_element.before_redraw(&device, &render_attachment, &mut command_encoder, &viewport);

    {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("CanvasLayer"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 0.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        element.redraw(&device, &mut render_pass, &viewport);
        capture_element.redraw(&device, &mut render_pass, &viewport);
    }

    capture_element.after_redraw(&device, &render_attachment, &mut command_encoder, &viewport);
    element.after_redraw(&device, &render_attachment, &mut command_encoder, &viewport);

    queue.submit([command_encoder.finish()]);

    capture_element.submit(&device);
    element.submit(&device);

    capture_element.detach();
    element.detach();

    device.poll(wgpu::MaintainBase::Wait);

    Ok(to_png(capture_fut.await?, viewport)?)
}

pub fn to_png(image_data: Vec<u8>, viewport: Viewport) -> Result<Png<'static>> {
    let mut png_data = Vec::<u8>::with_capacity(image_data.len());
    let mut encoder = png::Encoder::new(
        std::io::Cursor::new(&mut png_data),
        viewport.width,
        viewport.height,
    );
    encoder.set_color(png::ColorType::Rgba);
    let mut png_writer = encoder.write_header()?;
    png_writer.write_image_data(&image_data[..])?;
    png_writer.finish()?;
    log::info!("PNG file encoded in memory.");

    Ok(Png::Data(png_data.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::render::SvgElement;

    #[futures_test::test]
    async fn test_render_to_png() {
        let element = SvgElement::new(Some("render_to_png")).into();

        render_to_png(Some("render_to_png"), &element, Viewport::new(1920, 1080))
            .await
            .unwrap()
            .save("test.png")
            .await
            .unwrap();
    }
}
