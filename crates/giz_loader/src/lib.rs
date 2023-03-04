use std::{collections::HashMap, fs::File, path::PathBuf};

mod asset;
mod component;
mod context;
mod resource;

use asset::Asset;
use bevy_ecs::{component::TableStorage, world::World};
use context::Context;
use giz_core::Application;
use giz_math::{Children, GlobalTransform, Root, Transform};
use serde::Deserialize;
use serde_json::Value;

fn load_entity(
    context: &mut Context,
    world: &mut World,
    components: HashMap<String, Value>,
) {
    let mut entity = unsafe {
        let world_unsafe = &mut *(world as *mut World);
        world_unsafe.spawn_empty()
    };

    context
        .ids
        .insert(components["id"].as_u64().unwrap(), entity.id());

    for (component_name, value) in components {
        if component_name == "id" {
            continue;
        }

        component::load_component_for_entity(
            context,
            world,
            &mut entity,
            &component_name,
            value,
        );
    }
}

#[derive(Deserialize)]
struct WorldData {
    pub assets: Vec<String>,
    pub entities: Vec<HashMap<String, Value>>,
}

pub fn load_from_file(
    application: &mut Application,
    base_path: PathBuf,
    path_to_world: &str,
) -> anyhow::Result<World> {
    let data: WorldData =
        serde_json::from_reader(File::open(base_path.join(path_to_world))?)?;

    let assets = data
        .assets
        .iter()
        .map(|path| Asset::load(application, &base_path, path))
        .collect::<anyhow::Result<Vec<Asset>>>()?;

    let assets_map = assets
        .iter()
        .map(|asset| (asset.name.clone(), asset.deserialized.clone()))
        .collect::<HashMap<_, _>>();

    let mut context = Context {
        assets: assets_map,
        ids: HashMap::new(),
    };

    application.world.spawn((
        Root,
        Transform::default(),
        GlobalTransform::default(),
        Children(vec![]),
    ));

    for entity in data.entities {
        load_entity(&mut context, &mut application.world, entity);
    }

    Ok(World::new())
}
