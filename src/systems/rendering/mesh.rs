use bevy_ecs::prelude::*;

use crate::{
    components::{rendering::Mesh, GlobalTransform, Parent},
    resources::rendering::{GlResource, MaterialsResource},
};

pub fn mesh_render_system(
    gl: NonSend<GlResource>,
    materials: NonSend<MaterialsResource>,
    query: Query<(&Mesh, &GlobalTransform), With<Parent>>,
) {
    for material in &materials.0 {
        material.bind();

        for entity in &material.entities {
            let mesh = query.get_component::<Mesh>(*entity).unwrap();
            mesh.draw(&gl);
        }
    }
}
