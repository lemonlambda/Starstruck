pub mod renderer;

use std::thread::spawn;
use std::sync::{Arc, Mutex};

use renderer::Renderer;
use starry_ecs::resources::Resource;
use starry_ecs::World;
use winit::event_loop::EventLoopBuilder;
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};

#[derive(Clone)]
pub struct StarstruckEngine {
    renderer: Arc<Mutex<Option<Renderer>>>,
    app_name: String
}

impl std::fmt::Debug for StarstruckEngine {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("StarstruckEngine")
            .field("renderer", &"Mutex { ... }")
            .field("app_name", &self.app_name)
            .finish()
    }
}

impl StarstruckEngine {
    pub fn new(app_name: &str) -> Self {
        Self {
            renderer: Arc::new(Mutex::new(None)),
            app_name: String::from(app_name)
        }
    }
}

impl Resource for StarstruckEngine {}

pub fn run_winit(world: &World) {
    let world_clone = world.clone();
    
    spawn(move || { 
        let mut renderer = Renderer::new();
        let engine = world_clone.get_resource::<StarstruckEngine>();
        let eventloop = EventLoopBuilder::new().with_any_thread(true).build();
        let window = WindowBuilder::new().build(&eventloop).unwrap();
        renderer.create_instance(engine.app_name.clone(), &window).create_debug().create_physical_device();

        let binding = engine.renderer.clone();
        let mut locked = binding.lock().unwrap();
        *locked = Some(renderer);
        eventloop.run(move |event, _, control_flow| {
            control_flow.set_poll();
            control_flow.set_wait();

            window.request_redraw();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    control_flow.set_exit();
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                },
                Event::RedrawRequested(_) => {
                },
                _ => ()
            }
        });
    });
}