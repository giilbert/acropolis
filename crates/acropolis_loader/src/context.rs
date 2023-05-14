use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::prelude::Entity;

use crate::registry::Registry;

pub struct Context<'a> {
    pub assets: HashMap<String, Arc<Mutex<Option<Box<dyn Any>>>>>,
    pub entity_id_map: HashMap<u64, Entity>,
    pub registry: &'a Registry,
}
