use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Scriptable;
use bevy_ecs::prelude::*;
use deno_core::JsRuntime;

static BEHAVIOR_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Component, Scriptable, Default)]
pub struct Behavior {
    paths: Vec<String>,
}

unsafe impl Send for Behavior {}
unsafe impl Sync for Behavior {}

impl Behavior {
    pub fn new(paths: Vec<String>) -> Behavior {
        Behavior { paths }
    }

    pub fn run_create_script(
        &mut self,
        runtime: &mut JsRuntime,
        entity: Entity,
    ) {
        for path in &self.paths {
            runtime
                .execute_script(
                    path,
                    &format!(
                        "__acropolis__.createBehavior('{}', '{}', '{}');",
                        // "{{ let a = new {}(new Entity({})); behaviors[{}] = a; }}",
                        &path,
                        entity.index(),
                        BEHAVIOR_ID.fetch_add(1, Ordering::Relaxed),
                    ),
                )
                .expect("Error during script execution");
        }
    }
}
