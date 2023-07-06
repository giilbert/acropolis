use bevy_ecs::{
    component::ComponentId,
    prelude::{DetectChanges, Entity},
    world::World,
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
        method_id: u32,
        handle_scope: &mut v8::HandleScope,
        arguments: v8::Local<v8::Value>,
    ) {
        unimplemented!("call_component_method_mut {method_id}");
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

//  INFO  acropolis_scripting::init                   > {"acropolis_loader::resource::LoaderContextResource":0,"acropolis_loader::registry::Registry":1,"acropolis_render::resources::StateResource":2,"acropolis_scripting::resources::ScriptingExtensions":3,"acropolis_physics::resources::PhysicsResource":4,"acropolis_math::components::root::Root":5,"acropolis_math::components::transform::Transform":6,"acropolis_math::components::transform::GlobalTransform":7,"acropolis_math::components::transform::Children":8,"acropolis_render::components::mesh::Mesh":9,"acropolis_physics::components::collider2d::Collider2D":10,"acropolis_math::components::transform::Parent":11,"acropolis_physics::components::rigidbody2d::RigidBody2D":12,"acropolis_scripting::components::behavior::Behavior":13,"acropolis_render::components::camera::Camera":14,"acropolis_render::components::camera::CurrentCamera":15,"acropolis_scripting::resources::ScriptingResource":16}

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
