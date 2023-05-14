use std::borrow::Cow;

use crate::State;
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub struct Material {
    pub module: ShaderModule,
}

impl Material {
    pub fn new(state: &State, source: impl AsRef<str>) -> anyhow::Result<Self> {
        let module =
            state
                .lock()
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Wgsl(Cow::Borrowed(source.as_ref())),
                });

        Ok(Self { module })
    }

    // pub fn load() -> anyhow::Result<Self> {}
}

impl Drop for Material {
    fn drop(&mut self) {}
}
