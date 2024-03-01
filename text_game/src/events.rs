// events.rs

use crate::creatures::Creature;

pub struct TravelingDualChoice {
    pub location: String,
    pub description: String,
    pub choices: Vec<(String, String)>,
}

impl TravelingDualChoice {
    pub fn new(location: &str, description: &str, choices: &[(String, String)]) -> Self {
        TravelingDualChoice {
            location: location.to_string(),
            description: description.to_string(),
            choices: choices.to_vec(),
        }
    }

    pub fn display(&self) {
        println!("{}", self.description);
        for (i, choice) in self.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.0); // Print choice number and description
        }
    }
}


pub enum GameEvent {
    TravelingDualEvent(TravelingDualChoice),
    CombatEvent(Creature),
}