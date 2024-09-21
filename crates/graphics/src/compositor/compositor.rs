use std::{
    ops::{Deref, DerefMut},
    sync::mpsc,
};

use ecsrs::{AsComponent, Id};

use wgpu::{
    Adapter, Buffer, CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Extent3d,
    Queue, RenderPass, ShaderSource, Surface, SurfaceTarget, Texture, TextureDescriptor,
};

use crate::{
    compositor::{
        Canvas2DComponent, CaptureComponent, LayerComponent, RedrawComponent, SvgComponent,
    },
    BufferSizeOf, Error, Png, Result, Viewport,
};

use super::{RenderSystem, SvgSystem, SvgTessellated};

/// A builder for graphics [`Compositor`]
pub struct CompositorBuilder {
    svg_shader_source: ShaderSource<'static>,
}

impl CompositorBuilder {
    fn new() -> Self {
        Self {
            svg_shader_source: ShaderSource::Wgsl(include_str!("./system/shader/svg.wgsl").into()),
        }
    }

    async fn create_wgpu() -> Result<(Device, Queue)> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .ok_or(Error::RequestAdapterError)?;

        Ok(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WgpuCompositor"),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                    ..Default::default()
                },
                None,
            )
            .await?)
    }

    async fn create_wgpu_with<'window>(
        target: impl Into<SurfaceTarget<'window>>,
    ) -> Result<(Device, Queue, Adapter, Surface<'window>)> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(target)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(Error::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WgpuCompositor"),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                    ..Default::default()
                },
                None,
            )
            .await?;

        Ok((device, queue, adapter, surface))
    }

    async fn create(self, device: Device, queue: Queue) -> Result<Compositor> {
        let world = ecsrs::World::new([
            LayerComponent::component_type(),
            RedrawComponent::component_type(),
            Canvas2DComponent::component_type(),
            SvgComponent::component_type(),
            CaptureComponent::component_type(),
        ]);

        let systems: Vec<Box<dyn RenderSystem>> = vec![Box::new(SvgSystem::with_shader(
            Some("Svg"),
            &device,
            self.svg_shader_source,
        ))];

        Ok(Compositor {
            world,
            device,
            queue,
            systems,
        })
    }

    /// Create a [`Compositor`] whose rendering target is a `window`.
    pub async fn render_to_surface<'window>(
        self,
        target: impl Into<SurfaceTarget<'window>>,
        viewport: Viewport,
    ) -> Result<SurfaceCompositor<'window>> {
        let (device, queue, adapter, surface) = Self::create_wgpu_with(target).await?;
        let rendering = self.create(device, queue).await?;

        let config = surface
            .get_default_config(&adapter, viewport.width, viewport.height)
            .unwrap();

        surface.configure(&rendering.device, &config);

        Ok(SurfaceCompositor {
            surface,
            rendering,
            config,
        })
    }

    /// Create a [`Compositor`] whose rendering target is a GPU texture.
    pub async fn render_to_texture(self, viewport: Viewport) -> Result<TextureCompositor> {
        let (device, queue) = Self::create_wgpu().await?;

        let rendering = self.create(device, queue).await?;

        let texture_target = rendering.device.create_texture(&TextureDescriptor {
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

        let buffer = rendering.device.create_buffer(&wgpu::BufferDescriptor {
            size: viewport.buffer_size_of(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("TextureCompositor"),
            mapped_at_creation: false,
        });

        Ok(TextureCompositor {
            buffer,
            texture_target,
            rendering,
        })
    }
}

#[allow(unused)]
/// A stage for graphics rendering.
pub struct Compositor {
    /// rendering ecs world.
    world: ecsrs::World,
    /// Associated wgpu Device.
    device: Device,
    /// Associated wgpu Queue.
    queue: Queue,
    /// system for svg component.
    systems: Vec<Box<dyn RenderSystem>>,
}

impl Compositor {
    /// Create a rendering stage via [`CompositorBuilder`]
    pub fn new() -> CompositorBuilder {
        CompositorBuilder::new()
    }

    /// Create a new svg rendering element.
    ///
    /// On success, returns the id of new rendering element.
    pub fn new_svg(&mut self, tessellated: SvgTessellated) -> Id {
        let id = self.world.new_entity();

        self.world.new_component(
            SvgComponent {
                id: id.clone(),
                tessellated,
            },
            [&id],
        );

        id
    }
}

impl Compositor {
    fn prepare(&mut self, viewport: &Viewport, command_encoder: &mut CommandEncoder) {
        for system in &self.systems {
            system.prepare(&mut self.world, viewport, command_encoder);
        }
    }

    fn redraw<'a>(&mut self, viewport: &Viewport, render_pass: &mut RenderPass<'a>) {
        for system in &self.systems {
            system.redraw(&mut self.world, viewport, render_pass);
        }
    }

    fn composite(&mut self, viewport: &Viewport, command_encoder: &mut CommandEncoder) {
        for system in self.systems.iter().rev() {
            system.prepare(&mut self.world, viewport, command_encoder);
        }
    }

    fn submit<I: IntoIterator<Item = CommandBuffer>>(&mut self, command_buffers: I) {
        self.queue.submit(command_buffers);
    }
}

/// A rendering bound to a `window`
pub struct SurfaceCompositor<'window> {
    #[allow(unused)]
    /// surface associated with the window.
    surface: Surface<'window>,
    /// rendeing system
    rendering: Compositor,
    /// surface configuration.
    config: wgpu::SurfaceConfiguration,
}

impl<'window> Deref for SurfaceCompositor<'window> {
    type Target = Compositor;
    fn deref(&self) -> &Self::Target {
        &self.rendering
    }
}

impl<'window> DerefMut for SurfaceCompositor<'window> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rendering
    }
}

impl<'a> SurfaceCompositor<'a> {
    /// Invoke a rendering process and composite all rendering layer into the surface target.
    ///
    /// On success, this fn will call [`present`](wgpu::SurfaceTexture::present) internally.
    pub fn compositing(&mut self) -> Result<()> {
        let texture = self.surface.get_current_texture()?;

        let mut command_encoder =
            self.rendering
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("TextureCompositor"),
                });

        let viewport = Viewport::new(texture.texture.width(), texture.texture.height());

        let texture_view = texture.texture.create_view(&Default::default());

        self.prepare(&viewport, &mut command_encoder);

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("TextureCompositor"),
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

            self.redraw(&viewport, &mut render_pass);
        }

        self.composite(&viewport, &mut command_encoder);

        self.submit([command_encoder.finish()]);

        texture.present();

        Ok(())
    }

    /// Resize the surface's viewport.
    pub fn resize(&mut self, viewport: Viewport) {
        if viewport.width == 0 || viewport.height == 0 {
            log::warn!("SurfaceCompositor, resize with invalid data: {}", viewport);
            return;
        }

        self.config.width = viewport.width;
        self.config.height = viewport.height;

        self.surface.configure(&self.device, &self.config);
    }
}

/// A rendering bound to a `Texture`
pub struct TextureCompositor {
    /// surface associated with the window.
    texture_target: Texture,
    /// rendeing system
    rendering: Compositor,
    /// GPU buffer to copy texture.
    buffer: Buffer,
}

impl Deref for TextureCompositor {
    type Target = Compositor;
    fn deref(&self) -> &Self::Target {
        &self.rendering
    }
}

impl DerefMut for TextureCompositor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rendering
    }
}

impl TextureCompositor {
    /// Invoke a rendering process and composite all rendering layer into one GPU texture.
    ///
    /// On success, returns a png of rendering result.
    pub fn compositing(&mut self) -> Result<Png<'static>> {
        let mut command_encoder =
            self.rendering
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("TextureCompositor"),
                });

        let viewport = Viewport::new(self.texture_target.width(), self.texture_target.height());

        let texture_view = self.texture_target.create_view(&Default::default());

        self.prepare(&viewport, &mut command_encoder);

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("TextureCompositor"),
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

            self.redraw(&viewport, &mut render_pass);
        }

        self.composite(&viewport, &mut command_encoder);

        command_encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture_target,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(size_of::<u32>() as u32 * self.texture_target.width()),
                    rows_per_image: Some(self.texture_target.height()),
                },
            },
            Extent3d {
                width: self.texture_target.width(),
                height: self.texture_target.height(),
                depth_or_array_layers: 1,
            },
        );

        self.submit([command_encoder.finish()]);

        let (sender, receiver) = mpsc::channel();

        self.buffer
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                log::trace!("TextureCompositor, buffer map_async: {:?}", result);
                _ = sender.send(result);
            });

        self.device.poll(wgpu::MaintainBase::Wait);

        receiver.recv().unwrap()?;

        let view = self.buffer.slice(..).get_mapped_range();
        let image_data = view.to_vec();
        drop(view);
        self.buffer.unmap();

        to_png(image_data, viewport)
    }
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

    #[futures_test::test]
    async fn test_render_to_texture() {
        let mut compositor = Compositor::new()
            .render_to_texture(Viewport::new(256, 256))
            .await
            .unwrap();

        compositor.compositing().unwrap();

        compositor.compositing().unwrap();
    }
}
