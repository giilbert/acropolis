pub mod components;
mod plugin;
mod resources;
mod state;
mod systems;
mod utils;
mod window;

pub use plugin::RenderPlugin;
pub use resources::StateResource;
pub use state::{State, StateInner};
pub use utils::Material;
pub use window::Window;
