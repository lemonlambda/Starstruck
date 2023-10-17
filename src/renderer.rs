use crate::helpers::rgba;
use crate::shapes::{Vertex, Rectangle, Cube};

use wgpu::{Surface, Device, Queue, SurfaceConfiguration, Instance, InstanceDescriptor, Backends, TextureUsages, DeviceDescriptor, SurfaceError, TextureViewDescriptor, CommandEncoderDescriptor, Color, RenderPassColorAttachment, Operations, LoadOp, RenderPipeline, ShaderModuleDescriptor, ShaderSource, include_wgsl, Buffer};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window, event::WindowEvent};


pub struct Renderer {
    pub(crate) surface: Surface,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) surface_configuration: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) render_pipeline: RenderPipeline,

    // Window must be dropped after the surface
    pub(crate) window: Window,
    // Color/Vertices
    pub(crate) color: Color,
    pub(crate) num_vertices: u32,
    pub(crate) indices: u32,
    pub(crate) vertex_buffer: Buffer,
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

        let shader = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc()
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        let (rectangle, indices) = Rectangle::new((0.25, 0.25, 0.5), (0.5, 0.5, -0.5), 1.0, rgba(255, 180, 180, 255), rgba(180, 180, 255, 255)).rotate(45.0, 0.0, 0.0).into_raw();
        println!("{:#?}", rectangle);
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(rectangle.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );
        let (r, g, b, a) = rgba(255, 255, 255, 255);
        let clear_color = wgpu::Color {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: a.into(),
        };
        let num_vertices = rectangle.len() as u32;
        
        Self { 
            surface,
            device,
            queue,
            surface_configuration: config,
            size,
            render_pipeline,
            window,

            // Color/Vertices
            color: clear_color,
            indices,
            num_vertices,
            vertex_buffer
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
            let (r, g, b, a) = rgba(255, 255, 255, 255);
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: r.into(),
                            g: g.into(),
                            b: b.into(),
                            a: a.into(),
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..self.indices);
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
