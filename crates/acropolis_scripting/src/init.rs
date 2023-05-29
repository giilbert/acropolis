use crate::resources::{ScriptingExtensions, ScriptingResource};
use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;

use crate::components::Behavior;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __ACROPOLIS__)]
    fn set_component_ids(ids: String);
}

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

    let mut resource;
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            resource = ScriptingResource::new_wasm();
        } else {
            let extensions = extensions_resource.extensions.take().unwrap();
            resource = ScriptingResource::new_deno(extensions);
        }
    };

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

    let count = world.components().len();
    let component_id_enum_stuff = format!(
        "{{{}}}",
        world
            .components()
            .iter()
            .map(|c| (c.id().index(), c.name()))
            .map(|(i, name)| if i != count - 1 {
                format!("\"{}\":{},", name, i)
            } else {
                format!("\"{}\":{}", name, i)
            })
            .collect::<String>()
    );

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            set_component_ids(component_id_enum_stuff.clone());
        } else {
            resource
                .runtime
                .execute_script("acropolis-component-ids", &format!("const __ACROPOLIS_COMPONENT={};", component_id_enum_stuff))
                .unwrap();
        }
    };

    log::info!("{}", component_id_enum_stuff);

    world.insert_resource(resource);
}

pub fn init_scripting(
    loader_context: Res<acropolis_loader::LoaderContextResource>,
    mut scripting: ResMut<ScriptingResource>,
    mut query: Query<(&mut Behavior, Entity)>,
) {
    cfg_if::cfg_if! {
        if #[cfg(not(target_arch = "wasm32"))] {
            use std::fs;

            let bundle_source =
                fs::read_to_string(loader_context.root_path.join(".acropolis/out.js"))
                .expect("error reading bundle");

            let runtime = &mut scripting.runtime;
            runtime
                .execute_script("<acropolis-bundle>", &bundle_source)
                .expect("scripting failed to initialize");

            for (mut behavior, entity) in &mut query {
                behavior.run_create_script_deno(runtime, entity);
            }
        } else {
            for (mut behavior, entity) in &mut query {
                behavior.run_create_script_wasm(entity);
            }
        }
    }
}
