use bevy_ecs::schedule::StageLabel;

#[derive(StageLabel)]
pub enum Stage {
    Scripting,
    Update,
    Render,
}
