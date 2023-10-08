pub mod error;
pub mod prelude;
pub mod vulkan_instance;
pub mod plugins;
pub mod renderer;
pub mod helpers;

use std::ptr::null_mut;
use std::rc::Rc;

use crate::plugins::EnginePlugin;
use crate::error::EngineError;
use crate::renderer::Renderer;

pub struct Engine {
    internal: Rc<EngineInternal>
}

pub(crate) struct EngineInternal {
    renderer: Renderer,

    plugins: Vec<Box<dyn EnginePlugin>>
}

impl EngineInternal {
    pub(crate) fn new() -> Self {
        Self {
            renderer: Renderer::new(),

            plugins: vec![]
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            internal: Rc::new(EngineInternal::new())
        }
    }

    // Does a bunch of start up tasks
    pub fn start(mut self) -> Self {
        let cloned = self.internal.clone();
        for plugin in &cloned.plugins {
            plugin.plugin_make(&mut self.internal.clone());
        }
        self
    }

    pub fn run(mut self) -> ! {
        loop {
            let cloned = self.internal.clone();
            for plugin in &cloned.plugins {
                plugin.plugin_run(&mut self.internal.clone())
            }
        }
    }
}

pub struct EngineBase;

impl EnginePlugin for EngineBase {
    fn plugin_make(&self, engine: &mut Engine) {
        
    }
}