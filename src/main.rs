mod components;
mod lib;
mod resources;
mod systems;
// mod utils;

use lib::application::Application;

fn main() {
    pretty_env_logger::init();
    Application::new().run();
}
