use std::ptr::null_mut;

use ash::Instance;

pub struct Renderer {
    pub instance: Option<Instance>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            instance: None
        }
    }
}
