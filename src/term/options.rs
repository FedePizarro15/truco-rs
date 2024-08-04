use clap::Parser;
use serde::{Deserialize, Serialize};

use super::cli::Cli;

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    debug: bool,
    autosave: bool,
    selected_player: Option<u64>, // Jugador seleccionado por id.
    cli_mode: bool,
}

impl Options {
    fn new(cli: &Cli) -> Self {
        Self {
            debug: false,
            autosave: true,
            selected_player: None,
            cli_mode: false,
        }
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::create("options.json")?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    fn from_file() -> Result<Self, std::io::Error> {
        let file = std::fs::File::open("options.json")?;
        let reader = std::io::BufReader::new(file);
        let options: Options = serde_json::from_reader(reader)?;
        Ok(options)
    }
}

pub fn handle_options() {
    println!("Configurando opciones");
    let options = Options::from_file().unwrap_or_else(|_| Options::new(&Cli::parse()));
    println!("{:?}", options);
}