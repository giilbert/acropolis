use std::ops::{Deref, DerefMut};

use bevy_ecs::system::Resource;

use crate::State;

#[derive(Resource)]
pub struct StateResource(pub State);
impl Deref for StateResource {
    type Target = State;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for StateResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
