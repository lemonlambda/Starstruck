use std::thread::spawn;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;

use starry_ecs::resources::Resource;
use starry_ecs::World;
use wgpu::Instance;
use winit::event_loop::{EventLoopBuilder, EventLoop, ControlFlow};
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};

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
    pub async fn run(&mut self) -> &mut Self {
        run_winit().await;
        self
    }
}

impl Resource for StarstruckEngine {}

async fn run_winit() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();


}
