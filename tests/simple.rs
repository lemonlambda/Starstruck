use starry_ecs::World;
use starstruck::StarstruckEngine;

#[test]
fn adding_engine_resource() {
    World::new().add_resource(StarstruckEngine::new());
}
