use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::prelude::Entity;

use crate::{registry::Registry, Asset};

pub struct Context<'a> {
    pub assets: HashMap<String, Asset>,
    pub entity_id_map: HashMap<u64, Entity>,
    pub registry: &'a Registry,
}
