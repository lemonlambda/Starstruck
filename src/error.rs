use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Encountered Vulkan Error: `{0}`")]
    VulkanError(i32),
    #[error("Custom error: `{0}`")]
    Custom(String),
    #[error("Unknown error in engine.")]
    Unknown,
}
