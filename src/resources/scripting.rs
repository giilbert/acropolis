use bevy_ecs::prelude::*;
use deno_core::{op, serde_json, Extension, JsRuntime, RuntimeOptions};

use crate::{
    components::Transform, lib::scripting::scripting_api::ScriptingApi,
};

use super::rendering::StateResource;

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

#[op]
fn op_get_key_down(key: String) -> bool {
    let world = unsafe { &mut *SCRIPTING_WORLD.unwrap() };
    let state = world.resource::<StateResource>().lock();
    state.keys.contains(&serde_json::from_str(&key).unwrap())
}

pub struct ScriptingResource {
    pub runtime: JsRuntime,
}

impl ScriptingResource {
    pub fn new() -> Self {
        let extension = Extension::builder()
            .ops(vec![
                op_get_component_prop::decl(),
                op_set_component_prop::decl(),
                op_get_key_down::decl(),
            ])
            .build();
        let runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![extension],
            ..Default::default()
        });

        Self { runtime }
    }
}
