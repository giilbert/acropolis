use giz_core::{Plugin, Stage};
use giz_scripting::ScriptingExtensions;

use crate::Transform;

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&mut self, app: &mut giz_core::Application) {
        app.runtime_schedule.add_system_to_stage(
            Stage::Update,
            crate::systems::transform_propagate_system,
        );

        let mut extensions_resource =
            app.world.resource_mut::<ScriptingExtensions>();

        extensions_resource.register_component::<Transform>();
    }
}
