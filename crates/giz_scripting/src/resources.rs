// extremely cursed shit here

use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc};

use bevy_ecs::{
    component::{ComponentDescriptor, ComponentId},
    prelude::*,
};
use deno_core::{
    op, serde_json, Extension, ExtensionBuilder, JsRuntime, RuntimeOptions,
};
use serde::{Deserialize, Serialize};

use crate::Scriptable;

// TODO: make better & safer
pub static mut SCRIPTING_WORLD: Option<*mut World> = None;

unsafe fn get_scripting_api<'a>(
    entity: Entity,
    component_id: ComponentId,
) -> Option<&'a mut dyn Scriptable> {
    let world = &mut *SCRIPTING_WORLD.unwrap();
    let addr = {
        let mut component = world.get_mut_by_id(entity, component_id).unwrap();
        component.set_changed();
        component.into_inner().as_ptr() as *const ()
    };
    let extensions = world.resource::<ScriptingExtensions>();

    let o = extensions
        .components
        .get(&component_id)
        .unwrap()
        .scriptable_from_thin_ptr(addr);

    // let boxed: Box<dyn Scriptable> = Box::new(PhantomData);
    // assert_eq!(
    //     std::mem::size_of_val(&boxed),
    //     std::mem::size_of::<usize>() * 2
    // );
    // let (_, vtable) =
    //     std::mem::transmute_copy::<Box<_>, (*const u8, *const usize)>(&boxed);

    // let o = std::mem::transmute::<_, *mut dyn Scriptable>((
    //     addr as usize,
    //     vtable as usize,
    // ));

    Some(&mut *o)
}

#[op]
fn op_set_component_prop(
    entity_id: u32,
    component_id: usize,
    key: String,
    value: String,
) {
    let entity = Entity::from_raw(entity_id);
    if let Some(scripting_api) =
        unsafe { get_scripting_api(entity, ComponentId::new(component_id)) }
    {
        scripting_api.set_property(&key, value);
    }
}

#[op]
fn op_get_component_prop(
    entity_id: u32,
    component_id: usize,
    key: String,
) -> Option<String> {
    let entity = Entity::from_raw(entity_id);
    let scripting_api =
        unsafe { get_scripting_api(entity, ComponentId::new(component_id)) };
    Some(scripting_api?.get_property(&key))
}

// #[op]
// fn op_get_key_down(key: String) -> bool {
//     let world = unsafe { &mut *SCRIPTING_WORLD.unwrap() };
//     let state = world.resource::<StateResource>().lock();
//     state.keys.contains(&serde_json::from_str(&key).unwrap())
// }

#[derive(Resource)]
pub struct ScriptingResource {
    pub runtime: JsRuntime,
}

unsafe impl Send for ScriptingResource {}
unsafe impl Sync for ScriptingResource {}

impl ScriptingResource {
    pub fn new(mut more_extensions: Vec<Extension>) -> Self {
        let extension = Extension::builder()
            .ops(vec![
                op_get_component_prop::decl(),
                op_set_component_prop::decl(),
                // op_get_key_down::decl(),
            ])
            .build();

        let mut extensions = vec![extension];
        extensions.append(&mut more_extensions);

        let runtime = JsRuntime::new(RuntimeOptions {
            extensions,
            ..Default::default()
        });

        Self { runtime }
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
    pub extensions: Option<Vec<deno_core::Extension>>,
}

impl Default for ScriptingExtensions {
    fn default() -> Self {
        Self {
            registered_components: HashMap::new(),
            components: HashMap::new(),
            extensions: Some(vec![]),
        }
    }
}

impl ScriptingExtensions {
    pub fn add_extension(&mut self, extension_builder: &mut ExtensionBuilder) {
        self.extensions
            .as_mut()
            .expect("attempting to add extension after initialization")
            .push(extension_builder.build());
    }

    pub fn register_component<C: Component + Scriptable + Default>(&mut self) {
        log::info!("Registered component {}", std::any::type_name::<C>());
        let type_id = TypeId::of::<C>();

        unsafe {
            let b: Box<dyn Scriptable> = Box::new(C::default());
            let (_, vtable) =
                std::mem::transmute_copy::<Box<_>, (*const u8, *const ())>(&b);

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
