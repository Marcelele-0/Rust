// events.rs

use crate::creatures::Creature;

pub struct TravelChoice {
    pub location: String,
    pub description: String,
    pub choices: Vec<(String, String)>,
    pub encounter: Option<Creature>,
}

impl TravelChoice {
    pub fn new(location: &str, description: &str, choices: &[(String, String)], encounter:Option<Creature>) -> Self {
        TravelChoice {
            location: location.to_string(),
            description: description.to_string(),
            choices: choices.to_vec(),
            encounter, 
        }
    }

    pub fn display(&self) {
        println!("{}", self.description);
        for (i, choice) in self.choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice.0); 
        }
    }
}

pub enum GameEvent {
    TravelingEvent(TravelChoice),
    NeutralEncounterEvent(Creature),
}

/* To do
implement functiction in struct crature to determine if the creature is hostile or neutral
we can use hashmap to 

might like to adjust the hash map in main game loop becouse its limited to 2 choices
*/