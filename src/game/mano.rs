#![allow(dead_code)]
#![allow(unused)]

use serde::{Deserialize, Serialize};

use crate::check_if_folder_exists_and_create_if_not;

use super::carta::{Carta, Palo};

use super::mazo::MazoTruco;

trait Mano {
    fn new(cartas: Vec<Carta>) -> Self;
    fn random() -> Self;
    fn from_mazo(mazo: &mut MazoTruco) -> Self;
    fn calc_envido(cartas: Vec<Carta>) -> u8;
    // TODO: fn play_card(&mut self, carta: Carta) -> Option<Carta>;
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]

struct ManoTruco {
    cartas: Vec<Carta>,
    envido_value: u8,
    // TODO: flor_value: u8,
}

impl Mano for ManoTruco {
    fn new(cartas: Vec<Carta>) -> Self {
        let envido_value: u8 = Self::calc_envido(cartas.clone());
        Self { cartas, envido_value: 0 }
    }

    fn random() -> Self {
        let mut cartas: Vec<Carta> = Vec::new();

        for _ in 0..3 {
            cartas.push(Carta::random());
        }

        Self::new(cartas)
    }

    fn from_mazo(mazo: &mut MazoTruco) -> Self {
        let mut cartas: Vec<Carta> = Vec::new();

        for _ in 0..3 {
        }

        Self::new(cartas)
    }

    fn calc_envido(cartas: Vec<Carta>) -> u8 {
        let mut envido: u8 = 0;
        let mut values: Vec<(Palo, u8)> = Vec::new();

        for carta in cartas.iter() {
            let envido_card_value: u8 = carta.calc_envido_value();

            values.push((carta.palo, envido_card_value));
        }
        
        let mut possible_value: u8 = 0;

        for i in 0..values.len() {
            if values[i].1 > envido {
                envido = values[i].1;
            }

            for j in i+1..values.len() {
                if values[i].0 == values[j].0 {
                    possible_value = values[i].1 + values[j].1 + 20;

                    if possible_value > envido {
                        envido = possible_value;
                    }
                }
            }
        }

        envido
    }
    

    // TODO: fn play_card(&mut self, carta: Carta) -> Option<Carta> {
        
    //     None
    // }
}