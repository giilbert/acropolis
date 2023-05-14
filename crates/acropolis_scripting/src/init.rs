use std::any::Any;

use crate::resources::{ScriptingExtensions, ScriptingResource};
use bevy_ecs::prelude::*;

use crate::components::Behavior;

fn prepare_components(world: &mut World) {
    let descriptors = world
        .resource::<ScriptingExtensions>()
        .registered_components
        .iter()
        .map(|(_, (_, descriptor))| descriptor.clone())
        .collect::<Vec<_>>();

    for descriptor in descriptors {
        if world
            .components()
            .get_id(descriptor.borrow().as_ref().unwrap().type_id().unwrap())
            .is_some()
        {
            continue;
        }

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
        "const __ACROPOLIS_COMPONENT={{{}}};",
        world
            .components()
            .iter()
            .map(|c| (c.id().index(), c.name()))
            .map(|(i, name)| format!("\"{}\":{},", name, i))
            .collect::<String>()
    );

    resource
        .runtime
        .execute_script("acropolis-component-ids", &component_id_enum_stuff)
        .unwrap();

    log::info!("{}", component_id_enum_stuff);

    world.insert_resource(resource);
}

pub fn init_scripting(
    loader_context: Res<acropolis_loader::LoaderContextResource>,
    mut scripting: NonSendMut<ScriptingResource>,
    mut query: Query<(&mut Behavior, Entity)>,
) {
    use std::fs;

    let bundle_source =
        fs::read_to_string(loader_context.root_path.join(".acropolis/out.js"))
            .expect("error reading bundle");

    let runtime = &mut scripting.runtime;
    runtime
        .execute_script("<acropolis-bundle>", &bundle_source)
        .expect("scripting failed to initialize");

    for (mut behavior, entity) in &mut query {
        behavior.run_create_script(runtime, entity);
    }
}
