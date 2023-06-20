use std::{any::Any, collections::HashMap, sync::Arc};

use bevy_ecs::{prelude::Entity, system::Resource, world::World};
use serde_json::Value;

use crate::{asset::AssetMetadata, context::Context};

/// Called to load a component
pub type ComponentLoaderFunction = &'static (dyn Fn(&mut Context, &mut World, Entity, Value) -> anyhow::Result<()>
              + Send
              + Sync);

/// Called to load an asset
pub type AssetLoaderFunction = &'static (dyn Fn(
    &mut Context,
    &mut World,
    &Value,
    &[u8],
) -> anyhow::Result<Box<dyn Any + Send + Sync + 'static>>
              + Send
              + Sync);

/// Called on all entity creation
pub type EntityCreateFunction = &'static (dyn Fn(&mut Context, &mut World, Entity) -> anyhow::Result<()>
              + Send
              + Sync);

/// Called to set up entities a world
pub type InitFunction = &'static (dyn Fn(&mut Context, &mut World) -> anyhow::Result<()>
              + Send
              + Sync);

struct ComponentLoaderEntry {
    pub dependent_components: &'static [&'static str],
    pub function: ComponentLoaderFunction,
}

#[derive(Resource)]
pub struct Registry {
    component_functions: HashMap<String, ComponentLoaderEntry>,
    asset_functions: HashMap<String, AssetLoaderFunction>,
    entity_create_functions: Vec<EntityCreateFunction>,
    init_functions: Vec<InitFunction>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            component_functions: HashMap::new(),
            asset_functions: HashMap::new(),
            entity_create_functions: Vec::new(),
            init_functions: Vec::new(),
        }
    }

    pub fn register_component(
        &mut self,
        name: &str,
        dependent_components: &'static [&'static str],
        function: ComponentLoaderFunction,
    ) {
        self.component_functions.insert(
            name.to_string(),
            ComponentLoaderEntry {
                dependent_components,
                function,
            },
        );
    }

    pub fn register_asset(
        &mut self,
        name: &str,
        function: AssetLoaderFunction,
    ) {
        self.asset_functions.insert(name.to_string(), function);
    }

    pub fn register_entity_create(&mut self, function: EntityCreateFunction) {
        self.entity_create_functions.push(function);
    }

    pub fn register_init(&mut self, function: InitFunction) {
        self.init_functions.push(function);
    }

    pub fn init_world(
        &self,
        context: &mut Context,
        world: &mut World,
    ) -> anyhow::Result<()> {
        for function in &self.init_functions {
            function(context, world)?;
        }

        Ok(())
    }

    pub fn init_entity(
        &self,
        context: &mut Context,
        world: &mut World,
        entity: Entity,
    ) -> anyhow::Result<()> {
        for function in &self.entity_create_functions {
            function(context, world, entity)?;
        }

        Ok(())
    }

    pub fn load_asset(
        &self,
        context: &mut Context,
        world: &mut World,
        metadata: &AssetMetadata,
        data: &[u8],
    ) -> anyhow::Result<Arc<dyn Any + Send + Sync + 'static>> {
        let function = self
            .asset_functions
            .get(&metadata.asset_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No loader functions found for asset type: {}",
                    metadata.asset_type
                )
            })?;
        Ok(Arc::from(function(context, world, &metadata.rest, data)?))
    }

    pub fn load_component(
        &self,
        context: &mut Context,
        world: &mut World,
        entity: Entity,
        component: &str,
        data: Value,
        get_component_data: &impl Fn(&str) -> Option<Value>,
    ) -> anyhow::Result<()> {
        let entry =
            self.component_functions.get(component).ok_or_else(|| {
                anyhow::anyhow!(
                    "No loader functions found for component: {}",
                    component
                )
            })?;

        for dependent_component in entry.dependent_components {
            let dependent_component_data =
                get_component_data(dependent_component).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Dependent component {} not found for component {}",
                        dependent_component,
                        component
                    )
                })?;

            self.load_component(
                context,
                world,
                entity,
                dependent_component,
                dependent_component_data,
                get_component_data,
            )?;
        }

        (entry.function)(context, world, entity, data)
    }
}
