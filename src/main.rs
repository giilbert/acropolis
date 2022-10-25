// mod components;
mod lib;
mod resources;
// mod systems;
// mod utils;

use fern::colors::{Color, ColoredLevelConfig};
use lib::application::Application;

fn main() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .warn(Color::Yellow)
        .debug(Color::Yellow)
        .error(Color::Red)
        .trace(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [ {} ] {} - {}",
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::max())
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    Application::new().run();
}
