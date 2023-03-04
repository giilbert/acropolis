use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bevy_ecs::prelude::Entity;

pub struct Context {
    pub assets: HashMap<String, Arc<Mutex<Option<Box<dyn Any>>>>>,
    pub ids: HashMap<u64, Entity>,
}
