#![allow(dead_code)]
#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::check_if_folder_exists_and_create_if_not;

use super::carta::{Carta, Palo};


trait Mazo {
    fn new() -> Self;
    fn shuffle(&mut self);
    fn random() -> Self;
    fn draw(&mut self) -> Option<Carta>;
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct MazoTruco {
    cartas: Vec<Carta>,
}

impl Mazo for MazoTruco {
    fn new() -> Self {
        let mut cartas = Vec::new();
        for palo in [Palo::Espada, Palo::Basto, Palo::Copa, Palo::Oro].iter() {
            for valor in 1..=7 {
                cartas.push(Carta::new(*palo, valor));
            }
            for valor in 10..=12 {
                cartas.push(Carta::new(*palo, valor));
            }
        }
        Self { cartas }
    }

    fn shuffle(&mut self) {
        use rand::prelude::SliceRandom;
        self.cartas.shuffle(&mut rand::thread_rng());
    }

    fn random() -> Self {
        let mut mazo = Self::new();
        mazo.shuffle();
        mazo
    }

    fn draw(&mut self) -> Option<Carta> {
        self.cartas.pop()
    }
}

impl MazoTruco {
    pub fn save_to_file(&self, name: &str) -> Result<(), std::io::Error> {
        check_if_folder_exists_and_create_if_not("mazos")?;
        let file = std::fs::File::create(format!("mazos/{}.json", name))?;
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub fn from_file(name: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(format!("mazos/{}.json", name))?;
        let reader = std::io::BufReader::new(file);
        let mazo: MazoTruco = serde_json::from_reader(reader)?;
        Ok(mazo)
    }
}

impl std::fmt::Display for MazoTruco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for carta in self.cartas.iter() {
            write!(f, "{} ", carta.to_pretty_string())?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mazo() {
        let mazo = MazoTruco::new();
        assert_eq!(mazo.cartas.len(), 40);
    }

    #[test]
    fn test_shuffle_mazo() {
        let mut mazo = MazoTruco::new();
        let mut mazo2 = MazoTruco::new();
        mazo.shuffle();
        assert_ne!(mazo.cartas, mazo2.cartas);
    }

    #[test]
    fn test_draw_mazo() {
        let mut mazo = MazoTruco::new();
        let carta = mazo.draw();
        assert_eq!(mazo.cartas.len(), 39);
        assert!(carta.is_some());
    }
    #[test]
    fn test_random_mazo() {
        let mazo = MazoTruco::random();
        assert_eq!(mazo.cartas.len(), 40);
    }
    #[test]
    fn test_print_mazo() {
        let mazo = MazoTruco::new();
        println!("{}", mazo);
    }
    #[test]
    fn test_save_mazos() {
        let mazo = MazoTruco::new();
        mazo.save_to_file("test").unwrap();
        let mazo2 = MazoTruco::from_file("test").unwrap();
        assert_eq!(mazo, mazo2);

        let mazo = MazoTruco::random();
        mazo.save_to_file("random").unwrap();
        let mazo2 = MazoTruco::from_file("random").unwrap();
        assert_eq!(mazo, mazo2);
    }
}
