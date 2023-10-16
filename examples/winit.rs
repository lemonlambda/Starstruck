use starry_ecs::{World, systems::DefaultOrdering};
use starstruck::{StarstruckEngine, run_winit};


fn test_system(_world: &World) {
    // println!("Hello world");
}

fn main() {
    World::new().add_resource(StarstruckEngine::new("Winit Test")).add_system(DefaultOrdering::Run, test_system).add_startup_system(create_window).start().run();
}