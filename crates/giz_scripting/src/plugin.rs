use bevy_ecs::schedule::SystemStage;
use giz_core::{Plugin, Stage};
use giz_loader::Registry;

use crate::{
    init::{create_runtime, init_scripting},
    resources::{ScriptingExtensions, SCRIPTING_WORLD},
    systems::scripting_update_system,
};

pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&mut self, app: &mut giz_core::Application) {
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
            registry
                .register_component("behaviors", &|_, world, entity, value| {
                    Ok(())
                });
        });
    }
}
