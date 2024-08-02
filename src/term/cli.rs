pub const MAIN_MENU_STR: &str = r"  _______                               
 |       | .----. .--.--. .----. .-----.
 |.|   | | |   _| |  |  | |  __| |  _  |
 `-|.  |-' |__|   |_____| |____| |_____|
   |:  |                                
   |::.|                                
   `---'                                
";

use std::io::Error;

use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::{delete_all_dirs_recursively, game::jugador::Jugador, input, term::tui::button::TuiButton};

use super::tui::Tui;

#[derive(Parser, Debug)]
#[command(
    version = "v0.0.1",
    about = "Juego de truco para la terminal",
    long_about = "Queres jugar al truco, y parecer un hacker mientras lo haces?\nEntonces este es tu juego! 🃏\nTenes acceso a partidas rapidas, y a un modo campaña extenso con muchisimo contenido al estilo Nethack.\nPodras conquistar el pais antes de que los paraguayos dominen el pais?"
)]
pub struct Cli {
    #[command(subcommand)]
    mode: Commands,
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Jugar modo campaña.")]
    Campaign {},
    #[command(about = "Jugar partido rapido")]
    FastMatch {
        #[arg(short, long, default_value = "2")]
        player_count: u8,
    },
    #[command(about = "Jugar modo multijugador.")]
    Multiplayer {},
    #[command(about = "Configurar parametros.")]
    Options {},
    #[command(about = "Test.")]
    Test {},
    #[command(about = "Eliminar todos los datos (PELIGROSO)")]
    Reset,
}

#[derive(Debug, Serialize, Deserialize)]
struct Options {
    debug: bool,
    autosave: bool,
    selected_player: Option<u64>, // Jugador seleccionado por id.
    cli_mode: bool
}

impl Options {
    fn new(cli: &Cli) -> Self {
        Self {
            debug: false,
            autosave: true,
            selected_player: None,
            cli_mode: false
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



pub fn handle_cli(cli: Cli) -> Result<(), Error> {

    use std::thread::sleep;
    use std::time::Duration;
    let mut tui = Tui::new();
    tui.add_element(Box::new(TuiButton::default()));

    for y in 0..20 {
        tui.draw();
        tui.elements[0].change_position(None, Some(y));
        sleep(Duration::from_millis(2000));
    }
    
    match crossterm::event::read()? {
        _ => {}
    }


    match cli.mode {
        Commands::Campaign {} => {
            println!("Jugando modo campaña");
        }
        Commands::FastMatch { player_count } => {
            println!("Jugando partido rapido con {} jugadores", player_count);
        }
        Commands::Multiplayer {} => {
            println!("Jugando modo multijugador");
        }
        Commands::Options {} => {
            handle_options();
        }
        Commands::Test {} => {
            println!("Test");
            let nombre = input!("Ingrese nombre: ".white().bold());
            println!("Hola, {}", nombre);
            println!("{}", MAIN_MENU_STR);
        }
        Commands::Reset => {
            println!("{}", "Reseteando datos".red().bold());
            delete_all_dirs_recursively().expect("Error al borrar directorios");
        }
    }
    Ok(())
}

fn handle_options() {
    println!("Configurando opciones");
    let options = Options::from_file().unwrap_or_else(|_| Options::new(&Cli::parse()));
    println!("{:?}", options);
}