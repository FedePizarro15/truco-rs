use clap::Parser;
use truco_rs::term::cli::{handle_cli, Cli};



fn main() {
    let cli = Cli::parse();
    handle_cli(cli).expect("Error al manejar CLI");
}

