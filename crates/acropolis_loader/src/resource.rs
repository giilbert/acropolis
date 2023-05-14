use std::path::PathBuf;

use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct LoaderContextResource {
    pub root_path: PathBuf,
}
