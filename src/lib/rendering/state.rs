use std::{rc::Rc, sync::Arc};

use wgpu::{CommandEncoder, RenderPass};
use winit::{dpi::PhysicalSize, window::Window};

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: winit::window::Window,
}

impl State {
    pub async fn new(window: Window) -> State {
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

    // pub fn begin_render(&mut self) -> Result<(), wgpu::SurfaceError> {
    //     let output = self.surface.get_current_texture()?;
    //     let view = output
    //         .texture
    //         .create_view(&wgpu::TextureViewDescriptor::default());

    //     let mut encoder = self.device.create_command_encoder(
    //         &wgpu::CommandEncoderDescriptor {
    //             label: Some("Render Encoder"),
    //         },
    //     );

    //     {
    //         let render_pass =
    //             encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //                 label: None,
    //                 color_attachments: &[Some(
    //                     wgpu::RenderPassColorAttachment {
    //                         view: &view,
    //                         resolve_target: None,
    //                         ops: wgpu::Operations {
    //                             load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
    //                             store: true,
    //                         },
    //                     },
    //                 )],
    //                 depth_stencil_attachment: None,
    //             });

    //         self.render_pass = Some(render_pass);
    //     }

    //     Ok(())
    // }

    // pub fn end_render(
    //     &mut self,
    //     encoder: CommandEncoder,
    //     render_pass: RenderPass,
    // ) -> anyhow::Result<()> {
    //     let output = self.surface.get_current_texture()?;
    //     self.queue.submit(std::iter::once(encoder.finish()));
    //     output.present();

    //     Ok(())
    // }

    pub fn input(&self) -> bool {
        false
    }
}
