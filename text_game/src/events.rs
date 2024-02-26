// events.rs

use crate::Creature;

pub struct TravelingChoice {
    pub location: String,
    pub description: String,
    pub choice1: String,
    pub choice2: String,
    pub outcome1: String,
    pub outcome2: String,
}

impl TravelingChoice {
    pub fn new(location: &str, description: &str, choice1: &str, choice2: &str, outcome1: &str, outcome2: &str) -> Self {
        TravelingChoice {
            location: location.to_string(),
            description: description.to_string(),
            choice1: choice1.to_string(),
            choice2: choice2.to_string(),
            outcome1: outcome1.to_string(),
            outcome2: outcome2.to_string(),
        }
    }

    pub fn display(&self) {
        println!("\n{}", self.description);
        println!("1: {}", self.choice1);
        println!("2: {}", self.choice2);
    }
}

pub enum GameEvent {
    TravelingEvent(TravelingChoice),
    CombatEvent(Creature),
}
