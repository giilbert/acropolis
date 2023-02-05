use crate::resources::{ScriptingExtensions, ScriptingResource};
use bevy_ecs::prelude::*;
use giz_math::Transform;

use crate::components::Behavior;

const BOOTSTRAP_SOURCE: &str = include_str!("js/bootstrap.js");

fn prepare_components(world: &mut World) {
    let descriptors = world
        .resource::<ScriptingExtensions>()
        .registered_components
        .iter()
        .map(|(_, (_, descriptor))| descriptor.clone())
        .collect::<Vec<_>>();

    for descriptor in descriptors {
        world.init_component_with_descriptor(
            descriptor.borrow_mut().take().unwrap(),
        );
    }
}

pub fn create_runtime(world: &mut World) {
    prepare_components(world);

    let world_components = world
        .components()
        .iter()
        .map(|c| (c.id(), c.type_id().unwrap().to_owned()))
        .collect::<Vec<_>>();

    let mut extensions_resource = world.resource_mut::<ScriptingExtensions>();

    extensions_resource.register_component::<Transform>();
    extensions_resource.register_component::<Behavior>();

    let extensions = extensions_resource.extensions.take().unwrap();
    let mut resource = ScriptingResource::new(extensions);

    let ScriptingExtensions {
        ref registered_components,
        ref mut components,
        ..
    } = extensions_resource.as_mut();

    for (component_id, type_id) in world_components.iter() {
        match registered_components.get(type_id) {
            Some((vtable, _)) => {
                components.insert(*component_id, *vtable);
            }
            None => (),
        }
    }

    let component_id_enum_stuff = format!(
        "const __GIZ_COMPONENT={{{}}};",
        world
            .components()
            .iter()
            .map(|c| (c.id().index(), c.name()))
            .map(|(i, name)| format!("\"{}\":{},", name, i))
            .collect::<String>()
    );

    resource
        .runtime
        .execute_script("giz-component-ids", &component_id_enum_stuff)
        .unwrap();

    log::info!("{}", component_id_enum_stuff);

    world.insert_resource(resource);
}

pub fn init_scripting(
    mut scripting: NonSendMut<ScriptingResource>,
    mut query: Query<(&mut Behavior, Entity)>,
) {
    let runtime = &mut scripting.runtime;
    runtime
        .execute_script("giz-bootstrap", BOOTSTRAP_SOURCE)
        .expect("scripting failed to initialize");

    for (mut behavior, entity) in &mut query {
        behavior.run(runtime, entity);
    }
}
