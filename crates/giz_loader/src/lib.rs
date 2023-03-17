use std::{collections::HashMap, fs::File, path::PathBuf};

mod asset;
mod context;
mod plugin;
mod registry;
mod resource;

pub use context::Context;
pub use plugin::LoaderPlugin;
pub use registry::Registry;

use asset::Asset;
use bevy_ecs::world::World;
use giz_core::Application;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct WorldData {
    pub assets: Vec<String>,
    pub entities: Vec<HashMap<String, Value>>,
}

pub fn load_from_file(
    application: &mut Application,
    base_path: PathBuf,
    path_to_world: &str,
) -> anyhow::Result<()> {
    let data: WorldData =
        serde_json::from_reader(File::open(base_path.join(path_to_world))?)?;

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
