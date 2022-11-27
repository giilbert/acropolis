use parking_lot::{Mutex, MutexGuard};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use winit::{dpi::PhysicalSize, event::VirtualKeyCode, window::Window};

#[derive(Clone)]
pub struct State {
    inner: Arc<Mutex<StateInner>>,
}

impl State {
    pub async fn new(window: Window) -> Self {
        State {
            inner: Arc::new(Mutex::new(StateInner::new(window).await)),
        }
    }

    pub fn lock(&self) -> MutexGuard<StateInner> {
        self.inner.lock()
    }
}

pub struct StateInner {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: winit::window::Window,
    pub encoder: Option<wgpu::CommandEncoder>,
    pub view: Option<wgpu::TextureView>,
    pub keys: HashSet<VirtualKeyCode>,
}

impl StateInner {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            encoder: None,
            view: None,
            keys: Default::default(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}
