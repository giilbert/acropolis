use bevy_ecs::{component::ComponentId, prelude::*};
use deno_core::{
    op, serde_v8::Value, v8::HandleScope, Extension, ExtensionBuilder, JsRealm,
    JsRuntime, RuntimeOptions,
};
use serde::Deserialize;

use crate::{
    components::{Behavior, Transform},
    lib::scripting::scripting_api::ScriptingApi,
};

pub static mut SCRIPTING_WORLD: Option<*mut World> = None;

unsafe fn get_scripting_api<'a>(
    entity: Entity,
    component_id: u32,
) -> &'a mut dyn ScriptingApi {
    let world = &mut *SCRIPTING_WORLD.unwrap();

    match component_id {
        0 => world.get_mut::<Transform>(entity).unwrap().into_inner(),
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
    let scripting_api = unsafe { get_scripting_api(entity, component_id) };
    scripting_api.set_property(&key, value);
}

#[op]
fn op_get_component_prop(
    entity_id: u32,
    component_id: u32,
    key: String,
) -> String {
    let entity = Entity::from_raw(entity_id);
    let scripting_api = unsafe { get_scripting_api(entity, component_id) };
    return scripting_api.get_property(&key);
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
            ])
            .build();
        let runtime = JsRuntime::new(RuntimeOptions {
            extensions: vec![extension],
            ..Default::default()
        });

        Self { runtime }
    }
}
