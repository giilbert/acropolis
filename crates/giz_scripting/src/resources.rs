use bevy_ecs::prelude::*;
use deno_core::{
    op, serde_json, Extension, ExtensionBuilder, JsRuntime, RuntimeOptions,
};
use giz_core::components::Transform;
use serde::{Deserialize, Serialize};

use crate::ScriptingApi;

// TODO: make better & safer
pub static mut SCRIPTING_WORLD: Option<*mut World> = None;

unsafe fn get_scripting_api<'a>(
    entity: Entity,
    component_id: u32,
) -> Option<&'a mut dyn ScriptingApi> {
    let world = &mut *SCRIPTING_WORLD.unwrap();

    match component_id {
        0 => Some(world.get_mut::<Transform>(entity)?.into_inner()),
        _ => panic!(),
    }
}

#[op]
fn op_set_component_prop(
    entity_id: u32,
    component_id: u32,
    key: String,
    value: String,
) {
    let entity = Entity::from_raw(entity_id);
    if let Some(scripting_api) =
        unsafe { get_scripting_api(entity, component_id) }
    {
        scripting_api.set_property(&key, value);
    }
}

#[op]
fn op_get_component_prop(
    entity_id: u32,
    component_id: u32,
    key: String,
) -> Option<String> {
    let entity = Entity::from_raw(entity_id);
    let scripting_api = unsafe { get_scripting_api(entity, component_id) };
    Some(scripting_api?.get_property(&key))
}

// #[op]
// fn op_get_key_down(key: String) -> bool {
//     let world = unsafe { &mut *SCRIPTING_WORLD.unwrap() };
//     let state = world.resource::<StateResource>().lock();
//     state.keys.contains(&serde_json::from_str(&key).unwrap())
// }

#[derive(Resource)]
pub struct ScriptingResource {
    pub runtime: JsRuntime,
}

unsafe impl Send for ScriptingResource {}
unsafe impl Sync for ScriptingResource {}

impl ScriptingResource {
    pub fn new(mut more_extensions: Vec<Extension>) -> Self {
        let extension = Extension::builder()
            .ops(vec![
                op_get_component_prop::decl(),
                op_set_component_prop::decl(),
                // op_get_key_down::decl(),
            ])
            .build();

        let mut extensions = vec![extension];
        extensions.append(&mut more_extensions);

        let runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            ..Default::default()
        });

        Self { runtime }
    }
}

// TODO: find a more permenant place
#[derive(Serialize, Deserialize)]
struct JsVector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ScriptingApi for Transform {
    fn set_property(&mut self, name: &str, value: String) {
        match name {
            "position" => {
                let JsVector3 { x, y, z } =
                    serde_json::from_str(&value).unwrap();
                self.position.x = x;
                self.position.y = y;
                self.position.z = z;
            }
            "scale" => {
                let JsVector3 { x, y, z } =
                    serde_json::from_str(&value).unwrap();
                self.scale.x = x;
                self.scale.y = y;
                self.scale.z = z;
            }
            _ => panic!("bad property"),
        }
    }

    fn get_property(&self, name: &str) -> String {
        match name {
            "position" => {
                let payload = JsVector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: self.position.z,
                };
                serde_json::to_string(&payload).unwrap()
            }
            "scale" => {
                let payload = JsVector3 {
                    x: self.scale.x,
                    y: self.scale.y,
                    z: self.scale.z,
                };
                serde_json::to_string(&payload).unwrap()
            }
            _ => panic!("bad property"),
        }
    }
}

#[derive(Resource)]
pub struct ScriptingExtensions {
    pub extensions: Option<Vec<deno_core::Extension>>,
}

impl Default for ScriptingExtensions {
    fn default() -> Self {
        Self {
            extensions: Some(vec![]),
        }
    }
}

impl ScriptingExtensions {
    pub fn add_extension(&mut self, extension_builder: &mut ExtensionBuilder) {
        self.extensions
            .as_mut()
            .expect("attempting to add extension after initialization")
            .push(extension_builder.build());
    }
}

unsafe impl Send for ScriptingExtensions {}
unsafe impl Sync for ScriptingExtensions {}
