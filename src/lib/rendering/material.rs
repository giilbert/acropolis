use std::{borrow::Cow, cell::RefCell, error::Error, rc::Rc};

use crate::lib::rendering::State;
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub struct Material {
    state: State,
    module: ShaderModule,
}

impl Material {
    pub fn new(state: State, source: impl AsRef<str>) -> anyhow::Result<Self> {
        let module =
            state
                .lock()
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Wgsl(Cow::Borrowed(source.as_ref())),
                });

        Ok(Self { state, module })
    }
}

impl Drop for Material {
    fn drop(&mut self) {}
}
