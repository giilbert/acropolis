use std::{
    any::Any,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use giz_core::Application;
use giz_render::{Material, StateResource};
use serde_json::Value;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AssetUrl(pub String);

#[derive(Clone, Debug)]
pub struct Asset {
    pub name: String,
    pub asset_type: String,
    pub metadata_path: String,
    pub metadata: Value,
    pub data: Arc<Vec<u8>>,
    pub deserialized: Arc<Mutex<Option<Box<dyn Any>>>>,
}

fn load_from_data(
    app: &mut Application,
    asset_type: &str,
    data: &[u8],
) -> Box<dyn Any> {
    let state = &app.world.resource::<StateResource>().0;

    match asset_type {
        "material" => Box::new(
            Material::new(state, String::from_utf8_lossy(data)).unwrap(),
        ),
        _ => panic!("unrecognized asset type: {}", asset_type),
    }
}

impl Asset {
    pub fn load(
        application: &mut Application,
        base_path: &PathBuf,
        metadata_path: &str,
    ) -> anyhow::Result<Self> {
        let path = base_path.join(metadata_path);
        let metadata: Value = serde_json::from_reader(fs::File::open(&path)?)?;
        let data =
            fs::read(base_path.join(metadata["file"].as_str().ok_or_else(
                || anyhow::anyhow!("file field does not exist on asset."),
            )?))?;

        let asset_type = metadata["type"]
            .as_str()
            .ok_or_else(|| {
                anyhow::anyhow!("type field does not exist on asset.")
            })?
            .to_string();

        Ok(Self {
            name: metadata["name"]
                .as_str()
                .ok_or_else(|| {
                    anyhow::anyhow!("name field does not exist on asset.")
                })?
                .to_string(),
            metadata_path: metadata_path.to_string(),
            deserialized: Arc::new(Mutex::new(Some(load_from_data(
                application,
                &asset_type,
                &data,
            )))),
            data: Arc::new(data),
            asset_type,
            metadata,
        })
    }
}
