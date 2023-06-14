mod assets;
pub mod components;
mod plugin;
mod resources;
mod state;
mod systems;
mod window;

pub use assets::Material;
pub use plugin::RenderPlugin;
pub use resources::StateResource;
pub use state::{State, StateInner};
pub use window::Window;
