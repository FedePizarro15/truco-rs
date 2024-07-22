use std::path;

use serde::{Deserialize, Serialize};

use crate::{check_if_folder_exists_and_create_if_not, logln};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
enum Sexo {
    Masculino,
    Femenino,
}

impl std::fmt::Display for Sexo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sexo::Masculino => write!(f, "Masculino"),
            Sexo::Femenino => write!(f, "Femenino"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jugador {
    nombre: String,
    apodo: String,
    sexo: Sexo,
    dinero: f64,
    id: u64,
    // TODO: add in game properties.
}

impl Default for Jugador {
    fn default() -> Self {
        Jugador {
            nombre: String::from("Jugador"),
            apodo: String::new(),
            sexo: Sexo::Masculino,
            dinero: 0.0,
            id: 0,
        }
    }
}

impl PartialEq for Jugador {
    fn eq(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.apodo == other.apodo && self.sexo == other.sexo
    }
}

impl Jugador {
    fn new(nombre: String, apodo: String, sexo: Sexo, dinero: f64) -> Self {
        Jugador {
            nombre,
            apodo,
            sexo,
            dinero,
            id: rand::random(),
        }
    }
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let jugador: Jugador = serde_json::from_reader(reader)?;
        Ok(jugador)
    }
    pub fn save_to_file(&self) -> Result<(), std::io::Error> {
        check_if_folder_exists_and_create_if_not("players")?;
        let file = std::fs::File::create(format!("players/{}_player.json", self.apodo))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
    pub fn save_as_current(&self) -> Result<(), std::io::Error> {
        check_if_folder_exists_and_create_if_not("players")?;
        logln!("Saving player as current player");
        let path = std::path::Path::new("players/current_player.json");
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        let file = std::fs::File::create(path)?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        logln!("Player saved as current player");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jugador() {
        let jugador = Jugador::new(
            String::from("Juan Perez"),
            String::from("Juan"),
            Sexo::Masculino,
            100.0,
        );
        assert_eq!(jugador.nombre, "Juan Perez");
        assert_eq!(jugador.apodo, "Juan");
        assert_eq!(jugador.sexo, Sexo::Masculino);
        assert_eq!(jugador.dinero, 100.0);
    }

    #[test]
    fn test_jugador_default() {
        let jugador = Jugador::default();
        assert_eq!(jugador.nombre, "Jugador");
        assert_eq!(jugador.apodo, "");
        assert_eq!(jugador.sexo, Sexo::Masculino);
        assert_eq!(jugador.dinero, 0.0);
    }

    #[test]
    fn test_save_jugadores() {
        let jugador = Jugador::new(
            String::from("Juan Perez"),
            String::from("Juan"),
            Sexo::Masculino,
            100.0,
        );
        jugador.save_to_file().unwrap();
        let jugador2 = Jugador::from_file("players/Juan_player.json").unwrap();
        assert_eq!(jugador, jugador2);

        let jugador = Jugador::default();
        jugador.save_as_current().unwrap();
        let jugador2 = Jugador::from_file("players/current_player.json").unwrap();
        assert_eq!(jugador, jugador2);
    }
}
