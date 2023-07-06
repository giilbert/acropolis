use bevy_ecs::{component::ComponentId, prelude::Entity};
use deno_core::{op, v8};

use crate::scriptable::get_scripting_api_mut;

#[op(v8)]
pub fn op_call_component_method_mut(
    handle_scope: &mut v8::HandleScope,
    entity_id: u32,
    component_id: usize,
    key: u32,
    arguments: deno_core::serde_v8::Value,
) {
    let world = unsafe { &mut *crate::SCRIPTING_WORLD.unwrap() };
    let entity = Entity::from_raw(entity_id);

    if let Some(scripting_api) =
        unsafe { get_scripting_api_mut(entity, ComponentId::new(component_id)) }
    {
        scripting_api.call_component_method_mut(
            key,
            handle_scope,
            arguments.v8_value,
            world,
        );
    }
}
