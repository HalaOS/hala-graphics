use std::{
    sync::Arc,
    task::{Context, Poll, Waker},
};

use spin::Mutex;
use wgpu::{Buffer, CommandEncoder, Device, Extent3d, Texture};

use crate::Result;

static U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

enum InnerState {
    Init,
    Rendering(Arc<Buffer>),
    Sync(Result<Vec<u8>>),
}

#[derive(Default)]
struct RawCapture {
    waker: Option<Waker>,
    state: Option<InnerState>,
}

impl RawCapture {
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Result<Vec<u8>>> {
        if self.state.is_none() {
            self.state = Some(InnerState::Init);
            self.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }

        match self.state.take().unwrap() {
            InnerState::Sync(r) => return Poll::Ready(r),
            state => {
                self.state = Some(state);
                self.waker = Some(cx.waker().clone());
                return Poll::Pending;
            }
        }
    }

    fn capture(
        &mut self,
        device: &Device,
        command_encoder: &mut CommandEncoder,
        texture: &Texture,
        width: u32,
        height: u32,
    ) -> bool {
        if self.state.is_none() {
            return false;
        }

        if let Some(state) = self.state.as_ref() {
            match state {
                InnerState::Rendering(_) | InnerState::Sync(_) => return false,
                _ => {}
            }
        }

        let buffer_size = (U32_SIZE * width * height) as wgpu::BufferAddress;

        log::trace!("create capture({},{}), size={}", width, height, buffer_size);

        let desc = wgpu::BufferDescriptor {
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("CanvasLayer"),
            mapped_at_creation: false,
        };

        let buffer = Arc::new(device.create_buffer(&desc));

        command_encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(U32_SIZE * width),
                    rows_per_image: Some(height),
                },
            },
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.state = Some(InnerState::Rendering(buffer.clone()));

        return true;
    }

    fn sync(&mut self) -> Option<Arc<Buffer>> {
        if let Some(InnerState::Rendering(buffer)) = self.state.as_ref() {
            Some(buffer.clone())
        } else {
            None
        }
    }

    fn result(&mut self, result: Result<Vec<u8>>) -> Option<Waker> {
        if let Some(waker) = self.waker.take() {
            self.state = Some(InnerState::Sync(result));
            Some(waker)
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
pub(super) struct WgpuCapture(Arc<Mutex<RawCapture>>);

impl WgpuCapture {
    pub fn poll(&self, cx: &mut Context<'_>) -> Poll<Result<Vec<u8>>> {
        self.0.lock().poll(cx)
    }

    pub fn capture(
        &self,
        device: &Device,
        command_encoder: &mut CommandEncoder,
        texture: &Texture,
        width: u32,
        height: u32,
    ) -> bool {
        self.0
            .lock()
            .capture(device, command_encoder, texture, width, height)
    }

    pub fn sync(&self) {
        if let Some(buffer) = self.0.lock().sync() {
            let capturable = buffer.clone();

            let this = self.clone();

            buffer
                .slice(..)
                .map_async(wgpu::MapMode::Read, move |result| {
                    let waker = match result {
                        Ok(_) => {
                            let view = capturable.slice(..).get_mapped_range();

                            let buf = view.to_vec();

                            drop(view);

                            capturable.unmap();

                            this.result(Ok(buf))
                        }

                        Err(err) => this.result(Err(err.into())),
                    };

                    if let Some(waker) = waker {
                        waker.wake();
                    }
                });
        }
    }

    pub fn result(&self, result: Result<Vec<u8>>) -> Option<Waker> {
        self.0.lock().result(result)
    }
}
