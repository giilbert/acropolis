use std::sync::atomic::{AtomicUsize, Ordering};

use crate::Scriptable;
use bevy_ecs::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use deno_core::JsRuntime;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __ACROPOLIS__)]
    fn create_behavior(file_path: String, entity_id: u32, behavior_id: usize);
}

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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run_create_script_deno(
        &mut self,
        runtime: &mut JsRuntime,
        entity: Entity,
    ) {
        for path in &self.paths {
            runtime
                .execute_script(
                    path,
                    &format!(
                        "__ACROPOLIS__.scripting.createBehavior('{}', {}, {});",
                        // "{{ let a = new {}(new Entity({})); behaviors[{}] = a; }}",
                        &path,
                        entity.index(),
                        BEHAVIOR_ID.fetch_add(1, Ordering::Relaxed),
                    ),
                )
                .expect("Error during script execution");
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_create_script_wasm(&mut self, entity: Entity) {
        log::info!("Creating behavior");
        for path in &self.paths {
            create_behavior(
                path.clone(),
                entity.index(),
                BEHAVIOR_ID.fetch_add(1, Ordering::Relaxed),
            );
        }
    }
}
