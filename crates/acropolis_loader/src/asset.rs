use std::{
    any::Any,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bevy_ecs::world::World;
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
    #[serde(rename = "dependsOn", default = "Vec::new")]
    pub depends_on: Vec<String>,
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
    pub deserialized: Arc<Mutex<Option<Arc<dyn Any + Send + Sync + 'static>>>>,
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
            serde_json::from_slice(&crate::read_file(&path)?)?;
        let data = crate::read_file(&base_path.join(&metadata.file))?;

        for dependency in metadata.depends_on.iter() {
            if context.assets.contains_key(dependency) {
                continue;
            }

            let dependent_asset =
                Self::load(context, world, base_path, dependency)?;
            context
                .assets
                .insert(dependent_asset.name.clone(), dependent_asset);
        }

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

    pub fn is<T: Any>(&self) -> bool {
        let deserialized = self.deserialized.lock().unwrap();
        deserialized.is_some() && deserialized.as_ref().unwrap().is::<T>()
    }

    pub fn take_owned<T: Any + Send + Sync>(&self) -> Option<T> {
        let mut deserialized = self.deserialized.lock().ok()?;
        let deserialized = deserialized.take()?;
        let a = deserialized.downcast::<T>().ok()?;
        Arc::try_unwrap(a).ok()
    }

    pub fn get_ref<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        let deserialized = self.deserialized.lock().ok()?;
        let deserialized = deserialized.as_ref()?.clone();
        deserialized.downcast::<T>().ok()
    }
}
