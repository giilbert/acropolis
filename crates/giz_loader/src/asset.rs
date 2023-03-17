use std::{
    any::Any,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bevy_ecs::world::World;
use giz_core::Application;
use serde::Deserialize;
use serde_json::Value;

use crate::context::Context;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AssetUrl(pub String);

#[derive(Deserialize, Debug)]
pub struct AssetMetadata {
    pub file: String,
    #[serde(rename = "type")]
    pub asset_type: String,
    pub name: String,
    #[serde(flatten)]
    pub rest: Value,
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub asset_type: String,
    pub metadata_path: String,
    pub metadata: Value,
    pub data: Arc<Vec<u8>>,
    pub deserialized: Arc<Mutex<Option<Box<dyn Any>>>>,
}

impl Asset {
    pub fn load(
        context: &mut Context,
        world: &mut World,
        base_path: &PathBuf,
        metadata_path: &str,
    ) -> anyhow::Result<Self> {
        let path = base_path.join(metadata_path);
        let metadata: AssetMetadata =
            serde_json::from_reader(fs::File::open(&path)?)?;
        let data = fs::read(base_path.join(&metadata.file))?;

        Ok(Self {
            deserialized: Arc::new(Mutex::new(Some(
                context
                    .registry
                    .load_asset(context, world, &metadata, &data)?,
            ))),
            name: metadata.name,
            metadata_path: metadata_path.to_string(),
            data: Arc::new(data),
            asset_type: metadata.asset_type,
            metadata: metadata.rest,
        })
    }
}
