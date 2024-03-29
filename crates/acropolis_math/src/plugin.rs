use acropolis_core::{Plugin, Stage};
use acropolis_loader::Registry;
use acropolis_scripting::ScriptingExtensions;
use bevy_ecs::{prelude::*, system::EntityCommands, world::EntityMut};

use crate::{Children, GlobalTransform, Parent, Root, Transform};

pub struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&mut self, app: &mut acropolis_core::Application) {
        app.runtime_schedule.add_system_to_stage(
            Stage::Update,
            crate::systems::transform_propagate_system
                .label("transform_propagate"),
        );
        app.runtime_schedule
            .add_system_to_stage(Stage::Update, crate::systems::test_system);

        app.world.resource_scope::<ScriptingExtensions, _>(
            |_, mut extensions_resource| {
                extensions_resource.register_component::<Transform>();
            },
        );

        app.world.resource_scope::<Registry, _>(|_, mut registry| {
            registry.register_init(&|_ctx, world| {
                world.spawn((
                    Root,
                    Transform::default(),
                    GlobalTransform::default(),
                    Children(vec![]),
                ));

                Ok(())
            });

            registry.register_component(
                "Transform",
                &[],
                &|_, world, entity, value| {
                    let transform = Transform::from_json(world, value);
                    let global_transform = GlobalTransform::new();

                    let mut entity = world.entity_mut(entity);
                    entity.insert((transform, global_transform));

                    Ok(())
                },
            );

            registry.register_component(
                "Parent",
                &[],
                &|ctx, world, entity, value| {
                    let value: Option<u64> =
                        acropolis_scripting::serde_json::from_value(value)?;

                    match value {
                        Some(parent_id) => {
                            println!("some -> adding to {}", parent_id);

                            let parent_entity = ctx.entity_id_map[&parent_id];
                            world
                                .entity_mut(entity)
                                .insert(Parent(parent_entity));

                            let mut parent_entity =
                                world.get_entity_mut(parent_entity).unwrap();
                            let mut children =
                                parent_entity.get_mut::<Children>();

                            match children.as_mut() {
                                Some(children) => {
                                    children.0.push(entity);
                                }
                                None => {
                                    parent_entity
                                        .insert(Children(vec![entity]));
                                }
                            }
                        }
                        None => {
                            println!("none -> adding to root");
                            let (
                                (mut root_entity_children, root_entity_id),
                                ..,
                            ) = world
                                .query::<((&mut Children, Entity), With<Root>)>(
                                )
                                .single_mut(world);
                            root_entity_children.0.push(entity);

                            world
                                .entity_mut(entity)
                                .insert(Parent(root_entity_id));
                        }
                    }

                    Ok(())
                },
            );

            registry.register_after_load(&|context, world| Ok(()));
        });
    }
}
