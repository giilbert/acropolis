mod run;

use clap::Parser;
use colored::Colorize;

// acropolis -> run the project
// acropolis build -> build the project

#[derive(Debug, Parser)]
#[command(name = "acropolis")]
#[command(about = "Acropolis CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Parser)]
enum Commands {}

fn print_logo() {
    let bar = " ".on_white();
    let space = " ";
    let top_line = "Acropolis";
    // TODO:
    let bottom_line = "v0.0.0".purple();
    println!("{}{}{}{}  {top_line}", space, bar, space, bar);
    println!("{}{}{}{}  {bottom_line}", bar, space, bar, space);
}

fn main() {
    pretty_env_logger::init();
    print_logo();

    let args = Cli::parse();

    match args.command {
        None => {
            run::command();
        }
        _ => {
            todo!();
        }
    }
}
