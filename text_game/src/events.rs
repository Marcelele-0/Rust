// events.rs

use crate::Creature;

pub struct TravelingDualChoice {
    pub location: String,
    pub description: String,
    pub choice1: String,
    pub choice2: String,
    pub outcome1: String,
    pub outcome2: String,
}

impl TravelingDualChoice {
    pub fn new(location: &str, description: &str, choice1: &str, choice2: &str, outcome1: &str, outcome2: &str) -> Self {
        TravelingDualChoice {
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

pub struct TravelingTripleChoice {
    pub location: String,
    pub description: String,
    pub choice1: String,
    pub choice2: String,
    pub choice3: String,
    pub outcome1: String,
    pub outcome2: String,
    pub outcome3: String,
}

impl TravelingTripleChoice {
    pub fn new(location: &str, description: &str, choice1: &str, choice2: &str, choice3: &str, outcome1: &str, outcome2: &str, outcome3: &str) -> Self {
        TravelingTripleChoice {
            location: location.to_string(),
            description: description.to_string(),
            choice1: choice1.to_string(),
            choice2: choice2.to_string(),
            choice3: choice3.to_string(),
            outcome1: outcome1.to_string(),
            outcome2: outcome2.to_string(),
            outcome3: outcome3.to_string(),
        }
    }

    pub fn display(&self) {
        println!("\n{}", self.description);
        println!("1: {}", self.choice1);
        println!("2: {}", self.choice2);
        println!("3: {}", self.choice3);
    }
    
}

pub enum GameEvent {
    TravelingDualEvent(TravelingDualChoice),
    CombatEvent(Creature),
    TravelingTripleEvent(TravelingTripleChoice),
}
