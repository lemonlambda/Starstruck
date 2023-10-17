pub mod renderer;

use renderer::Renderer;
use winit::dpi::PhysicalSize;

use std::thread::spawn;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;

use starry_ecs::resources::Resource;
use starry_ecs::World;
use wgpu::Instance;
use winit::event_loop::{EventLoopBuilder, EventLoop, ControlFlow};
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent, ElementState, VirtualKeyCode, KeyboardInput};

#[derive(Clone)]
pub struct StarstruckEngine {
    app_name: String,
    world: World
}

impl std::fmt::Debug for StarstruckEngine {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("StarstruckEngine")
            .field("app_name", &self.app_name)
            .finish()
    }
}

impl StarstruckEngine {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: String::from(app_name),
            world: World::new()
        }
    }
    pub async fn run(self) {
        let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();

        let mut renderer = Renderer::new(window).await;

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == renderer.window().id() => if !renderer.input(event) { // UPDATED!
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            let PhysicalSize { width: x, height: y } = *physical_size;
                            renderer.resize(x, y);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let PhysicalSize { width: x, height: y } = **new_inner_size;
                            renderer.resize(x, y);
                        }
                        _ => {}
                    }
                },
                Event::RedrawRequested(window_id) if window_id == renderer.window().id() => {
                    renderer.update();
                    match renderer.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size.width, renderer.size.height),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => {}
            }
        })
    }
}

impl Resource for StarstruckEngine {}
