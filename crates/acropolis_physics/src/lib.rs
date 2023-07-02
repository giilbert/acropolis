mod components;
mod resources;
mod systems;

use acropolis_core::{Application, Plugin, Stage};
use bevy_ecs::prelude::*;
use components::{collider2d::Collider2D, rigidbody2d::RigidBody2D};

use deno_core::serde_json;
use rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&mut self, app: &mut Application) {
        app.world.insert_resource(resources::PhysicsResource::new());
        app.runtime_schedule.add_system_set_to_stage(
            Stage::Update,
            SystemSet::new()
                .with_system(
                    systems::tick::update_positions_pre_tick
                        .before("tick")
                        .after("transform_propagate"),
                )
                .with_system(
                    systems::tick::tick_system
                        .label("tick")
                        .after("transform_propagate"),
                )
                .with_system(
                    systems::tick::update_positions_post_tick
                        .after("tick")
                        .after("transform_propagate"),
                ),
        );

        app.world.resource_scope::<acropolis_loader::Registry, _>(
            |_, mut registry| {
                registry.register_component(
                    "Collider2D",
                    &[],
                    &|_ctx, world, entity, value| {
                        let collider = Collider2D::load(
                            world
                                .get_resource::<resources::PhysicsResource>()
                                .unwrap()
                                .clone(),
                            value,
                        );

                        world.entity_mut(entity).insert(collider);

                        Ok(())
                    },
                );

                registry.register_component(
                    "RigidBody2D",
                    &["Collider2D"],
                    &|_ctx, world, entity, value| {
                        let rigidbody = RigidBody2D::load(
                            world
                                .get_resource::<resources::PhysicsResource>()
                                .unwrap()
                                .clone(),
                            value,
                            &mut *world.get_mut::<Collider2D>(entity).unwrap(),
                        );

                        world.entity_mut(entity).insert(rigidbody);

                        Ok(())
                    },
                );
            },
        );
    }
}
