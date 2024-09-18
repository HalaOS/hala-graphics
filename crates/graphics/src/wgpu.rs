use wgpu::{Device, Queue};

use crate::{Error, Result};

/// Create new wgpu context.
pub(crate) async fn init_wgpu() -> Result<(Device, Queue)> {
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
