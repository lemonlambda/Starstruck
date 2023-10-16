use starstruck::StarstruckEngine;

#[tokio::test]
async fn adding_engine_resource() {
    StarstruckEngine::new("Simple Test").run().await;
}
