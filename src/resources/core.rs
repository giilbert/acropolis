use bevy_ecs::entity::Entity;

pub struct Root {
    pub entity: Entity,
}

impl Root {
    pub fn new(entity: Entity) -> Root {
        Root { entity }
    }
}
