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
}

unsafe impl Send for Behavior {}
unsafe impl Sync for Behavior {}

impl Behavior {
    pub fn new(name: String, class_name: String) -> Behavior {
        Behavior { name, class_name }
    }

    pub fn run(&mut self, runtime: &mut JsRuntime, entity: Entity) {
        runtime
            .execute_script(
                &self.name,
                &format!(
                    "{{ let a = new {}(new Entity({})); behaviors[{}] = a; }}",
                    self.class_name,
                    // TODO: make more unique
                    entity.index(),
                    entity.index()
                ),
            )
            .expect("Error during script execution");
    }
}
