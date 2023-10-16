use starstruck::StarstruckEngine;

#[tokio::main]
async fn main() {
    StarstruckEngine::new("Simple Test").run().await;
}
