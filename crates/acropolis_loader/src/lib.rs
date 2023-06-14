use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

mod asset;
mod context;
mod plugin;
mod registry;
mod resource;

pub use context::Context;
pub use plugin::LoaderPlugin;
pub use registry::Registry;
pub use resource::LoaderContextResource;

use acropolis_core::Application;
use asset::Asset;
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
struct WorldData {
    pub assets: Vec<String>,
    pub entities: Vec<HashMap<String, Value>>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = __ACROPOLIS__)]
    fn js_read_file(path: &str) -> Option<Vec<u8>>;
}

pub fn read_file(path: &Path) -> anyhow::Result<Vec<u8>> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            Ok(
                js_read_file(path.to_str().unwrap()).ok_or_else(|| anyhow::anyhow!("Not found"))?
            )
        } else {
            Ok(std::fs::read(path)?)
        }
    }
}

pub fn load_from_file(
    application: &mut Application,
    base_path: PathBuf,
    path_to_world: PathBuf,
) -> anyhow::Result<()> {
    let data: WorldData =
        serde_json::from_slice(&read_file(&base_path.join(path_to_world))?)?;

    application
        .world
        .resource_scope::<LoaderContextResource, _>(|_world, mut context| {
            context.root_path = base_path.clone();
        });

    application
        .world
        .resource_scope::<Registry, _>(|world, registry| {
            let mut context = Context {
                assets: HashMap::new(),
                entity_id_map: HashMap::new(),
                registry: registry.as_ref(),
            };

            let assets = data
                .assets
                .iter()
                .map(|path| Asset::load(&mut context, world, &base_path, path))
                .collect::<anyhow::Result<Vec<Asset>>>()?;

            let assets_map = assets
                .iter()
                .map(|asset| (asset.name.clone(), asset.deserialized.clone()))
                .collect::<HashMap<_, _>>();

            context.assets = assets_map;

            registry.init_world(&mut context, world)?;

            for entity_definition in data.entities {
                let entity = world.spawn_empty().id();
                registry.init_entity(&mut context, world, entity)?;

                context.entity_id_map.insert(
                    entity_definition["id"].as_u64().ok_or_else(|| {
                        anyhow::anyhow!("Entity does not have an id field.")
                    })?,
                    entity,
                );

                for (component, data) in entity_definition.iter() {
                    if component == "id" {
                        continue;
                    }

                    registry.load_component(
                        &mut context,
                        world,
                        entity,
                        component,
                        data.clone(),
                    )?;
                }
            }

            Ok::<_, anyhow::Error>(())
        })?;

    Ok(())
}
