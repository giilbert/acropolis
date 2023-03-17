use giz_core::{components::Name, Application, Plugin};

use crate::registry::Registry;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&mut self, app: &mut Application) {
        app.world.insert_resource(Registry::new());

        app.world.resource_scope::<Registry, _>(|_, mut registry| {
            registry.register_component(
                "Name",
                &|_ctx, world, entity, value| {
                    let name = Name(
                        value
                            .as_str()
                            .ok_or(anyhow::anyhow!(
                                "Expected string for name."
                            ))?
                            .to_string(),
                    );
                    let mut entity = world.entity_mut(entity);
                    entity.insert(name);

                    Ok(())
                },
            );
        });
    }
}
