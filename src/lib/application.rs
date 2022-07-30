use std::process::Child;

use crate::components::transform::{Children, GlobalTransform, Parent};
use crate::resources::core::Root;
use crate::systems::transform::transform_propagate_system;
use crate::utils::types::Vector3;
use crate::{components::transform::Transform, resources::*};
use bevy_ecs::prelude::{Component, Query, With};
use bevy_ecs::{
    prelude::World,
    schedule::{Schedule, SystemStage},
};

use super::window::Window;
use glow::HasContext;

#[derive(Component)]
pub struct Name(pub String);

fn test_system(mut query: Query<(&GlobalTransform, &mut Transform, &Name)>) {
    for (transform, mut local_transform, name) in &mut query {
        println!("{} -- {:?}", name.0, transform.matrix);
        if name.0 == "parent".to_string() {
            local_transform.set_position(Vector3::new(0.0, 0.0, 3.0));
        }

        if name.0 == "child".to_string() {
            local_transform.set_position(Vector3::new(0.0, 0.0, 2.0));
        }
    }
}

pub struct Application {
    window: Window,
    pub world: World,
    schedule: Schedule,
}

impl Application {
    pub fn new() -> Application {
        let window = Window::new();

        let mut world = World::default();
        world.insert_non_send_resource(rendering::RenderingResource {
            gl: window.gl.clone(),
        });

        let root_id = {
            let mut root = world.spawn();
            root.insert(Transform::new())
                .insert(GlobalTransform::new())
                .insert(Name("root".to_string()))
                .id()
        };
        let parent_id = {
            let mut parent = world.spawn();
            parent
                .insert(Transform::new())
                .insert(GlobalTransform::new())
                .insert(Parent(root_id))
                .insert(Name("parent".to_string()))
                .insert(Children(vec![]))
                .id()
        };
        let child_id = {
            let mut child = world.spawn();
            child
                .insert(Transform::new())
                .insert(GlobalTransform::new())
                .insert(Parent(parent_id))
                .insert(Name("child".to_string()))
                .insert(Children(vec![]))
                .id()
        };
        let child_child_id = {
            let mut child = world.spawn();
            child
                .insert(Transform::new())
                .insert(GlobalTransform::new())
                .insert(Parent(child_id))
                .insert(Name("child of child".to_string()))
                .insert(Children(vec![]))
                .id()
        };

        world
            .get_entity_mut(parent_id)
            .unwrap()
            .insert(Children(vec![child_id]));

        world
            .get_entity_mut(child_id)
            .unwrap()
            .insert(Children(vec![child_child_id]));

        world.insert_resource(Root::new(root_id));
        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::single_threaded()
                .with_system(test_system)
                .with_system(transform_propagate_system),
        );

        unsafe {
            let gl = window.gl.clone();
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }

        Application {
            window,
            world,
            schedule,
        }
    }

    pub fn run(mut self) {
        let gl = self.window.gl.clone();

        let update = move || {
            unsafe {
                gl.clear(glow::COLOR_BUFFER_BIT);
            }

            self.schedule.run_once(&mut self.world);
        };

        self.window.run_event_loop(Box::new(update));
    }
}
