mod components;
mod lib;
mod resources;
mod systems;
mod utils;

use lib::application::Application;

fn main() {
    Application::new().run();
}
