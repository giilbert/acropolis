use bevy_ecs::{entity::Entity, system::Resource};

#[derive(Resource)]
pub struct Root {
    pub entity: Entity,
}

impl Root {
    pub fn new(entity: Entity) -> Root {
        Root { entity }
    }
}
