use bevy_ecs::{
    component::ComponentId,
    prelude::{DetectChanges, Entity},
};
use deno_core::{serde_v8, v8};

use crate::{ScriptingExtensions, SCRIPTING_WORLD};

pub trait Scriptable {
    fn set_property(&mut self, name: &str, value: String) {
        unimplemented!("set_property {name}: {value}")
    }

    fn get_property(&self, name: &str) -> String {
        unimplemented!("get_property {name}")
    }

    fn set_property_vec3(&mut self, property: u32, x: f64, y: f64, z: f64) {
        unimplemented!("set_property_vec3 {property}: {x}, {y}, {z}")
    }

    fn get_property_vec3(&self, property: u32) -> (f64, f64, f64) {
        unimplemented!("get_property_vec3 {property}")
    }

    #[allow(unused_variables)]
    fn call_component_method_mut(
        &mut self,
        property: u32,
        handle_scope: &mut v8::HandleScope,
        arguments: v8::Local<v8::Value>,
    ) {
        unimplemented!("call_component_method_mut {property}");
    }
}

pub unsafe fn get_scripting_api<'a>(
    entity: Entity,
    component_id: ComponentId,
) -> Option<&'a dyn Scriptable> {
    let world = &mut *SCRIPTING_WORLD.unwrap();
    let addr = {
        let component = world.get_mut_by_id(entity, component_id).unwrap();
        component.into_inner().as_ptr() as *const ()
    };

    let extensions = world.resource::<ScriptingExtensions>();

    let o = extensions
        .components
        .get(&component_id)
        .expect("component not scriptable")
        .scriptable_from_thin_ptr(addr);

    Some(o)
}

pub unsafe fn get_scripting_api_mut<'a>(
    entity: Entity,
    component_id: ComponentId,
) -> Option<&'a mut dyn Scriptable> {
    let world = &mut *SCRIPTING_WORLD.unwrap();
    let addr = {
        let mut component = world.get_mut_by_id(entity, component_id).unwrap();
        component.set_changed();
        component.into_inner().as_ptr() as *const ()
    };

    let extensions = world.resource::<ScriptingExtensions>();

    let o = extensions
        .components
        .get(&component_id)
        .expect("component not scriptable")
        .scriptable_from_thin_ptr(addr);

    Some(o)
}
