use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Instance, InstanceDescriptor, Backends, TextureUsages, DeviceDescriptor, SurfaceError, TextureViewDescriptor, CommandEncoderDescriptor, Color, RenderPassColorAttachment};
use winit::{dpi::PhysicalSize, window::Window, event::WindowEvent};

pub struct Renderer {
    pub(crate) surface: Surface,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) surface_configuration: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,

    // Window must be dropped after the surface
    pub(crate) window: Window
}
 
impl Renderer {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|adapter| {
                // Check if this adapter supports our surface
                adapter.is_surface_supported(&surface)
            })
            .unwrap();
        
        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        
        Self { 
            surface,
            device,
            queue,
            surface_configuration: config,
            size,
            window
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn input(&self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, x: u32, y: u32) {
        if x > 0 {
            self.size = PhysicalSize {
                width: x,
                height: self.size.height
            };
            self.surface_configuration.width = x;
            self.surface.configure(&self.device, &self.surface_configuration);
        }
        if y > 0 {
            self.size = PhysicalSize {
                width: self.size.width,
                height: y
            };
            self.surface_configuration.height = y;
            self.surface.configure(&self.device, &self.surface_configuration);
        }
    }
}
