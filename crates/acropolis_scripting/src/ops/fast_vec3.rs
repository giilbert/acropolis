use bevy_ecs::{component::ComponentId, prelude::Entity};

use crate::scriptable::{get_scripting_api, get_scripting_api_mut};

#[macros::glued_function]
pub fn op_set_component_vec3_prop(
    entity_id: u32,
    component_id: usize,
    key: u32,
    x: f64,
    y: f64,
    z: f64,
) {
    let entity = Entity::from_raw(entity_id);
    if let Some(scripting_api) =
        unsafe { get_scripting_api_mut(entity, ComponentId::new(component_id)) }
    {
        scripting_api.set_property_vec3(key, x, y, z);
    }
}

#[macros::glued_function]
pub fn op_get_component_vec3_prop(
    entity_id: u32,
    component_id: usize,
    key: u32,
) -> Option<(f64, f64, f64)> {
    let entity = Entity::from_raw(entity_id);
    let scripting_api =
        unsafe { get_scripting_api(entity, ComponentId::new(component_id)) };
    Some(scripting_api?.get_property_vec3(key))
}
