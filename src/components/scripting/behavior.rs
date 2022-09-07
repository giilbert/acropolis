use bevy_ecs::prelude::*;
use deno_core::{
    v8::{Global, Value},
    JsRuntime,
};

use crate::resources::scripting::ScriptingResource;

#[derive(Component)]
pub struct Behavior {
    pub name: String,
    pub class_name: String,
    inner: Option<Global<Value>>,
}

unsafe impl Send for Behavior {}
unsafe impl Sync for Behavior {}

impl Behavior {
    pub fn new(name: String, class_name: String) -> Behavior {
        Behavior {
            name,
            class_name,
            inner: None,
        }
    }

    pub fn run(&mut self, runtime: &mut JsRuntime, entity: Entity) {
        let return_val = runtime
            .execute_script(
                &self.name,
                &format!(
                    "new {}(new Entity({}))",
                    self.class_name,
                    entity.id()
                ),
            )
            .expect("Error during script execution");

        self.inner = Some(return_val);
    }
}
