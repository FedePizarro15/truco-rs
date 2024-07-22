#![allow(dead_code)]
#![allow(unused)]
use serde::{Deserialize, Serialize};

use crate::Parse;

#[derive(Debug)]
pub enum CartaError {
    InvalidValue,
    InvalidPalo,
    InvalidRelativeValue,
    
}

//TODO: Una buena idea aca es conseguir de forma dinamica la cantidad de palos, a traves de un archivo de configuracion.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Palo {
    Espada,
    Basto,
    Copa,
    Oro,
}

impl std::fmt::Display for Palo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Palo::Espada => write!(f, "Espada"),
            Palo::Basto => write!(f, "Basto"),
            Palo::Copa => write!(f, "Copa"),
            Palo::Oro => write!(f, "Oro"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Carta {
    palo: Palo,
    valor: u8,
    relative_value: u8,
    //TODO: add in game properties.
}

impl Carta {
    pub const fn new(palo: Palo, valor: u8) -> Self {
        let relative_value = match (valor, palo) {
            (4, _) => 1,
            (5, _) => 2,
            (6, _) => 3,
            (7, Palo::Basto | Palo::Copa) => 4,
            (10, _) => 5,
            (11, _) => 6,
            (12, _) => 7,
            (1, Palo::Copa | Palo::Oro) => 8,
            (2, _) => 9,
            (3, _) => 10,
            (7, Palo::Oro) => 11,
            (7, Palo::Espada) => 12,
            (1, Palo::Basto) => 13,
            (1, Palo::Espada) => 14,
            _ => 0,
        };
        Carta {
            palo,
            valor,
            relative_value,
        }
    }

    pub fn random() -> Self {
        let palo = match rand::random::<u8>()
            .checked_rem(4)
            .expect("Error al generar carta random (palo)")
        {
            0 => Palo::Espada,
            1 => Palo::Basto,
            2 => Palo::Copa,
            3 => Palo::Oro,
            _ => unreachable!(),
        };
        let valor = match rand::random::<u8>()
            .checked_rem(12)
            .expect("Error al generar carta random (valor)")
        {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 5,
            5 => 6,
            6 => 7,
            7 => 10,
            8 => 11,
            9 => 12,
            10 => 1,
            11 => 7,
            _ => unreachable!(),
        };
        Self::new(palo, valor)
    }

    pub fn to_pretty_string(&self) -> String {
        use owo_colors::OwoColorize;
        format!(
            "{}{}{}",
            self.valor.white().bold().italic().underline(),
            " de ".white().bold().underline(),
            match self.palo {
                Palo::Espada => "Espada".bright_blue().bold().underline().to_string(),
                Palo::Basto => "Basto".bright_green().bold().underline().to_string(),
                Palo::Copa => "Copa".bright_red().bold().underline().to_string(),
                Palo::Oro => "Oro".bright_yellow().bold().underline().to_string(),
            }
        )
    }
}

impl PartialEq for Carta {
    fn eq(&self, other: &Self) -> bool {
        self.relative_value == other.relative_value
    }
}

impl Eq for Carta {}

impl PartialOrd for Carta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) 
    }
}

impl Ord for Carta {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.relative_value.cmp(&other.relative_value)
    }
}


// Con esto puedo convertir cualquier cosa que pueda ser interpretada como una &str en una Carta.
impl<T: AsRef<str>> Parse<T> for Carta {
    type Error = CartaError;

    fn parse(value: T) -> Result<Self, Self::Error> {
        let value = value.as_ref();
        let lower = value.trim().to_lowercase();
        let value = lower.split_whitespace().collect::<Vec<&str>>();
        if value.len() != 3 { // Debe de estar formateada como "<num> de <palo>"
            return Err(CartaError::InvalidValue);
        }
        let valor = match value[0].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Err(CartaError::InvalidValue),
        };
        let palo = match value[2] {
            "espada" => Palo::Espada,
            "basto" => Palo::Basto,
            "copa" => Palo::Copa,
            "oro" => Palo::Oro,
            _ => return Err(CartaError::InvalidPalo),
        };
        Ok(Carta::new(palo, valor))
    }
}

impl std::fmt::Display for Carta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} de {}", self.valor, self.palo)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let carta = Carta::new(Palo::Espada, 1);
        assert_eq!(carta.palo, Palo::Espada);
        assert_eq!(carta.valor, 1);
        assert_eq!(carta.relative_value, 14);
    }
    #[test]
    fn test_print() {
        let carta = Carta::random();
        println!("{}", carta.to_pretty_string());
        assert_eq!(carta.to_pretty_string(), Carta::parse(carta.to_string()).unwrap().to_pretty_string());
    }
}
