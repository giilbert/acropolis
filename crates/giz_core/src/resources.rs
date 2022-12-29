use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource)]
pub struct Root(pub Entity);
