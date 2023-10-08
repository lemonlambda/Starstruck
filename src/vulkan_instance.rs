use crate::helpers::make_api_version;
use crate::{prelude::EngineError, Engine, plugins::{EnginePlugin, EngineCleanup}};

/// Makes a vulkan instance and manages the state around it
pub struct VulkanInstancePlugin;

impl EnginePlugin for VulkanInstancePlugin {
    fn plugin_make(&self, engine: &mut Engine) {
        self.create_instance(engine).unwrap();
    }
}

impl VulkanInstancePlugin {
    fn new() -> Self {
        Self
    }
    fn create_instance(&self, engine: &mut Engine) -> Result<(), EngineError> {
        todo!()
    }
}

impl EngineCleanup for VulkanInstancePlugin {}
