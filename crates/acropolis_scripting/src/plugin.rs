use bevy_ecs::schedule::SystemStage;
use acropolis_core::{Plugin, Stage};
use acropolis_loader::Registry;

use crate::{
    init::{create_runtime, init_scripting},
    resources::{ScriptingExtensions, SCRIPTING_WORLD},
    systems::scripting_update_system,
    Behavior,
};

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&mut self, app: &mut acropolis_core::Application) {
        unsafe { SCRIPTING_WORLD = Some(app.world.as_mut() as *mut _) };
        app.world.init_resource::<ScriptingExtensions>();

        app.init_schedule.add_stage(
            "scripting::init",
            SystemStage::single_threaded()
                .with_system(create_runtime)
                .with_system(init_scripting),
        );

        app.runtime_schedule
            .stage(Stage::Scripting, |stage: &mut SystemStage| {
                stage.add_system(scripting_update_system)
            });

        app.world.resource_scope::<Registry, _>(|_, mut registry| {
            registry.register_component(
                "behaviors",
                &|_ctx, world, entity, value| {
                    let paths = value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| {
                            v.get("src").unwrap().as_str().unwrap().to_string()
                        })
                        .collect::<Vec<_>>();

                    let mut entity = world.entity_mut(entity);
                    entity.insert(Behavior::new(paths));

                    Ok(())
                },
            );
        });
    }
}
