mod lib;
mod resources;
mod systems;

use bevy_ecs::prelude::*;
use glow::HasContext;

use lib::window::Window;
use resources::*;

fn hello_world_system() {
    println!("Hello world!");
}

fn main() {
    let window = Window::new();

    let mut world = World::default();
    world.insert_non_send_resource(rendering::RenderingResource {
        gl: window.gl.clone(),
    });

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update",
        SystemStage::parallel().with_system(hello_world_system),
    );

    let gl = window.gl.clone();

    unsafe {
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let update = move || {
        schedule.run_once(&mut world);

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    };

    window.run_event_loop(Box::new(update));
}
