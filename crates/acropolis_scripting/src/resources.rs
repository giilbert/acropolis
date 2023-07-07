// extremely cursed shit here

use hashbrown::HashMap;
use std::{any::TypeId, cell::RefCell, mem::MaybeUninit, rc::Rc};

use bevy_ecs::{
    component::{ComponentDescriptor, ComponentId},
    prelude::*,
};

use crate::Scriptable;

// TODO: make better & safer
pub static mut SCRIPTING_WORLD: Option<*mut World> = None;

#[derive(Resource)]
pub struct ScriptingResource {
    #[cfg(not(target_arch = "wasm32"))]
    pub runtime: deno_core::JsRuntime,
}

unsafe impl Send for ScriptingResource {}
unsafe impl Sync for ScriptingResource {}

impl ScriptingResource {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_deno(mut more_extensions: Vec<deno_core::Extension>) -> Self {
        use deno_core::{Extension, JsRuntime, RuntimeOptions};
        let extension = Extension::builder()
            .ops(crate::ops::deno_get_all_props())
            .build();

        let mut extensions = vec![extension];
        extensions.append(&mut more_extensions);

        let runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            ..Default::default()
        });

        Self { runtime }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_wasm() -> Self {
        Self {}
    }
}

#[derive(Copy, Clone)]
pub struct ScriptingVTable(pub *const ());

impl ScriptingVTable {
    pub unsafe fn scriptable_from_thin_ptr(
        &self,
        ptr: *const (),
    ) -> &mut dyn Scriptable {
        std::mem::transmute((ptr, self.0))
    }
}

#[derive(Resource)]
pub struct ScriptingExtensions {
    pub registered_components: HashMap<
        TypeId,
        (ScriptingVTable, Rc<RefCell<Option<ComponentDescriptor>>>),
    >,
    pub components: HashMap<ComponentId, ScriptingVTable>,
    #[cfg(not(target_arch = "wasm32"))]
    pub extensions: Option<Vec<deno_core::Extension>>,
}

impl Default for ScriptingExtensions {
    fn default() -> Self {
        Self {
            registered_components: HashMap::new(),
            components: HashMap::new(),
            #[cfg(not(target_arch = "wasm32"))]
            extensions: Some(vec![]),
        }
    }
}

impl ScriptingExtensions {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_extension(
        &mut self,
        extension_builder: &mut deno_core::ExtensionBuilder,
    ) {
        self.extensions
            .as_mut()
            .expect("attempting to add extension after initialization")
            .push(extension_builder.build());
    }

    pub fn register_component<C: Component + Scriptable>(&mut self) {
        log::info!("Registered component {}", std::any::type_name::<C>());
        let type_id = TypeId::of::<C>();

        unsafe {
            // just using this box to get the vtable
            let c: C = MaybeUninit::zeroed().assume_init();
            let b: Box<dyn Scriptable> = Box::new(c);

            let (_, vtable) =
                std::mem::transmute_copy::<Box<_>, (*const u8, *const ())>(&b);

            // prevent the box from being dropped, since it contains all zeros
            Box::leak(b);

            self.registered_components.insert(
                type_id,
                (
                    ScriptingVTable(vtable),
                    Rc::new(RefCell::new(
                        Some(ComponentDescriptor::new::<C>()),
                    )),
                ),
            );
        }
    }
}

unsafe impl Send for ScriptingExtensions {}
unsafe impl Sync for ScriptingExtensions {}
