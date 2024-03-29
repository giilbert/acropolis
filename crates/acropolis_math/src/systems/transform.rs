use bevy_ecs::prelude::*;
use nalgebra::Matrix4;

use crate::{Children, GlobalTransform, Parent, Root, Transform};

pub fn test_system(mut query: Query<&mut Transform>) {
    // for mut transform in &mut query {
    //     transform.position.x += 0.001;
    // }
}

pub fn transform_propagate_system(
    root: Query<Entity, With<Root>>,
    mut changed_local_transform_query: Query<
        (&Transform, Entity, Option<&Children>, Option<&Parent>),
        (Changed<Transform>, Without<Root>),
    >,
    mut global_transform_query: Query<(&mut GlobalTransform, &Transform)>,
    children_query: Query<&Children>,
) {
    for (transform_component, entity, children, parent) in
        &mut changed_local_transform_query
    {
        let parent_entity = match parent {
            Some(parent) => parent.0,
            None => root.single(),
        };
        let parent_transform = global_transform_query
            .get_component::<GlobalTransform>(parent_entity)
            .expect(
                "every entity requires a parent transform component. not found",
            );

        if parent_entity == entity {
            panic!("parent and entity are the same");
        }

        let matrix = transform_component
            .generate_matrix_parent(&parent_transform.matrix);

        {
            let mut global_transform = global_transform_query
                .get_component_mut::<GlobalTransform>(entity)
                .expect("every entity requires a global transform. not found");
            global_transform.matrix = matrix;
        }

        if let Some(children) = children {
            propagate_children_recursive(
                &mut global_transform_query,
                &children_query,
                children,
                &matrix,
            );
        }
    }
}

fn propagate_children_recursive(
    global_transform_query: &mut Query<(&mut GlobalTransform, &Transform)>,
    children_query: &Query<&Children>,
    children: &Children,
    parent_matrix: &Matrix4<f32>,
) {
    for child in &children.0 {
        let local_matrix = global_transform_query
            .get_component::<Transform>(*child)
            .expect(
                "every entity requires a local transform component. not found",
            )
            .generate_matrix();

        let global_matrix = {
            let mut global_transform_component = global_transform_query
                .get_component_mut::<GlobalTransform>(*child)
                .expect("every entity requires a global transform component. not found");
            let matrix = global_transform_component
                .generate_matrix_parent(&local_matrix, parent_matrix);
            global_transform_component.matrix = matrix;

            matrix
        };

        let children = children_query.get(*child);

        if let Ok(children) = children {
            propagate_children_recursive(
                global_transform_query,
                children_query,
                children,
                &global_matrix,
            );
        }
    }
}
