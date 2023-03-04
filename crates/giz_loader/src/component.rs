use bevy_ecs::{
    prelude::Entity,
    query::With,
    world::{EntityMut, World},
};
use giz_math::{Children, GlobalTransform, Parent, Root, Transform};
use giz_render::components::{Camera, CurrentCamera, Mesh};

use crate::context::Context;

pub fn load_component_for_entity(
    ctx: &mut Context,
    world: &mut World,
    entity: &mut EntityMut,
    component_name: &str,
    value: serde_json::Value,
) {
    match component_name {
        "Transform" => entity
            .insert(Transform::from_json(world, value))
            .insert(GlobalTransform::default()),
        "Parent" => {
            if value.is_null() {
                let ((mut root_entity_children, root_entity_id), ..) = world
                    .query::<((&mut Children, Entity), With<Root>)>()
                    .single_mut(world);

                entity.insert(Parent(root_entity_id));
                root_entity_children.0.push(entity.id());

                return;
            }

            let parent_id = value.as_u64().unwrap();
            let parent_entity = ctx.ids[&parent_id];
            entity.insert(Parent(parent_entity));
            entity
        }
        "Children" => {
            let children = value
                .as_array()
                .unwrap()
                .iter()
                .map(|child| ctx.ids[&child.as_u64().unwrap()])
                .collect::<Vec<_>>();
            entity.insert(Children(children));
            entity
        }
        "Mesh" => entity.insert(Mesh::from_json(&ctx.assets, world, value)),
        "Root" => entity.insert(Root),
        "Camera" => entity.insert(Camera::from_json(world, value)),
        "CurrentCamera" => entity.insert(CurrentCamera),
        _ => panic!("Unknown component: {}", component_name),
    };
}
