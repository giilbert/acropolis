use giz_core::{Plugin, Stage};

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&mut self, app: &mut giz_core::Application) {
        app.runtime_schedule.add_system_to_stage(
            Stage::Update,
            crate::systems::transform_propagate_system,
        );
    }
}
