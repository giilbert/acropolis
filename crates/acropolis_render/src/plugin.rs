use bevy_ecs::prelude::*;
use acropolis_core::Plugin;
use acropolis_loader::Registry;

use crate::{
    components::{Camera, CurrentCamera, Mesh},
    resources::StateResource,
    systems::{camera_view_matrix_update_system, mesh_render_system},
    Material, Window,
};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&mut self, app: &mut acropolis_core::Application) {
        app.runtime_schedule.add_stage(
            "render",
            SystemStage::parallel()
                .with_system(mesh_render_system)
                .with_system(camera_view_matrix_update_system),
        );

        let window = pollster::block_on(Window::new());

        app.world
            .insert_resource(StateResource(window.state.clone()));

        app.world.resource_scope::<Registry, _>(|_, mut registry| {
            registry.register_asset(
                "Material",
                &|_ctx, world, _value, bytes| {
                    let state = world.resource_mut::<StateResource>();
                    let material =
                        Material::new(&*state, String::from_utf8_lossy(bytes))?;
                    Ok(Box::new(material))
                },
            );

            registry.register_component(
                "Mesh",
                &|ctx, world, entity, value| {
                    let state = world.resource_mut::<StateResource>().clone();
                    let state = state.lock();

                    let mut entity = world.entity_mut(entity);
                    entity.insert(Mesh::load(ctx, &state, value));

                    Ok(())
                },
            );

            registry.register_component(
                "Camera",
                &|_ctx, world, entity, value| {
                    let state = world.resource_mut::<StateResource>().clone();
                    let state = state.lock();

                    let mut entity = world.entity_mut(entity);
                    entity.insert(Camera::from_json(&state, value));

                    Ok(())
                },
            );

            registry.register_component(
                "CurrentCamera",
                &|_, world, entity, _| {
                    world.entity_mut(entity).insert(CurrentCamera);
                    Ok(())
                },
            )
        });

        app.runner = Box::new(move |mut app| {
            let state = window.state.clone();

            window.run_event_loop(move || {
                let frame = {
                    let mut state = state.lock();

                    app.world.resource_mut::<StateResource>().set_changed();

                    let frame = state
                        .surface
                        .get_current_texture()
                        .expect("Failed to acquire next swap chain texture");
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    state.view = Some(view);
                    state.encoder = Some(state.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("Command Encoder"),
                        },
                    ));

                    frame
                };

                app.runtime_schedule.run(&mut app.world);

                {
                    let mut state = state.lock();
                    let commands = state.encoder.take().unwrap().finish();
                    state.queue.submit(Some(commands));
                    frame.present();
                }
            });
        });
    }
}
