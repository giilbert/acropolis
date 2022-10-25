use std::{borrow::Cow, cell::RefCell, error::Error, rc::Rc};

use crate::lib::rendering::State;
use bevy_ecs::prelude::*;
use uuid::Uuid;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

pub struct Material {
    state: Rc<RefCell<State>>,
    module: ShaderModule,
}

impl Material {
    pub fn new(
        state: Rc<RefCell<State>>,
        source: impl AsRef<str>,
    ) -> anyhow::Result<Self> {
        let module = state.borrow().device.create_shader_module(
            ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Wgsl(Cow::Borrowed(source.as_ref())),
            },
        );

        Ok(Self { state, module })
    }
}

impl Drop for Material {
    fn drop(&mut self) {}
}
