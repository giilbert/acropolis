use std::borrow::Cow;

use crate::State;
use acropolis_loader::Context;
use acropolis_scripting::serde_json::{self, Value};
use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

use super::Texture;

#[derive(serde::Deserialize)]
pub struct MaterialData {
    pub texture: String,
}

pub struct Material {
    pub module: ShaderModule,
    pub texture: Texture,
}

impl Material {
    pub fn load(
        state: &State,
        source: impl AsRef<str>,
        ctx: &mut Context,
        data: &Value,
    ) -> anyhow::Result<Self> {
        let module =
            state
                .lock()
                .device
                .create_shader_module(ShaderModuleDescriptor {
                    label: None,
                    source: ShaderSource::Wgsl(Cow::Borrowed(source.as_ref())),
                });

        let data = serde_json::from_value::<MaterialData>(data.clone())?;
        let texture = ctx
            .assets
            .get(&data.texture)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Texture {} not found in asset registry",
                    data.texture
                )
            })?
            .take_owned::<Texture>()
            .ok_or_else(|| {
                anyhow::anyhow!("Texture {} is not a texture", data.texture)
            })?;

        Ok(Self { module, texture })
    }

    // pub fn load() -> anyhow::Result<Self> {}
}

impl Drop for Material {
    fn drop(&mut self) {}
}
