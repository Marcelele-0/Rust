struct GameState {
    current_location: String,
}

impl GameState {
    fn new(start_location: &str) -> Self {
        GameState {
            current_location: start_location.to_string(),
        }
    }

    fn transition(&mut self, new_location: &str) {
        self.current_location = new_location.to_string();
    }
}
#[derive(Clone)]
struct Creature {
    name: String,
    health: u32,
    attack: u32,
    defense: u32,
}


impl Creature {
    fn new(name: &str, health: u32, attack: u32, defense: u32) -> Self {
        Creature {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }
    fn fight(&mut self, other: &mut Creature) {
        
        if self.attack >= other.defense {
            let attacks_to_kill = other.health / (self.attack - other.defense).max(1);
            let attacks_to_die = self.health / (other.attack - self.defense).max(1); 
            
            if attacks_to_kill <= attacks_to_die {
                println!("{} wins over {} in a deadly battle and learned through the fight.", self.name, other.name);
                self.attack += other.attack / 2;
                self.health = self.health-((attacks_to_kill - 1) * other.attack) + other.health / 2;
                self.defense += other.defense / 2;
                
                println!("{}'s remaining health is {}", self.name, self.health);
                println!("{}'s new attack is {}", self.name, self.attack);
                println!("{}'s new defense is {}", self.name, self.defense);

            } else {
                println!("{} was too strong for {}, you lose.", other.name, self.name); 
            }
        } else {
            println!("{} is too strong for {}", other.name, self.name);
        }
    }    
}


struct TravelingChoice {
    location: String,
    description: String,
    choice1: String,
    choice2: String,
    outcome1: String,
    outcome2: String,
}

impl TravelingChoice {
    fn new(location: &str, description: &str, choice1: &str, choice2: &str, outcome1: &str, outcome2: &str) -> Self {
        TravelingChoice {
            location: location.to_string(),
            description: description.to_string(),
            choice1: choice1.to_string(),
            choice2: choice2.to_string(),
            outcome1: outcome1.to_string(),
            outcome2: outcome2.to_string(),
        }
    }

    fn display(&self) {
        println!("{}", self.description);
        println!("1: {}", self.choice1);
        println!("2: {}", self.choice2);
    }
}
enum GameEvent {
    TravelingEvent(TravelingChoice),
    CombatEvent(Creature),
}

use std::collections::HashMap;

fn main() {
    let mut game_events: HashMap<String, GameEvent> = HashMap::new();
    let mut game_state = GameState::new("Start");

    game_events.insert("Start".to_string(), GameEvent::TravelingEvent(TravelingChoice::new(
        "Start",
        "You are at a crossroad. Which path will you take?",
        "Go to Castle",
        "Go to Forest",
        "Castle",
        "Forest",
    )));

    game_events.insert("Castle".to_string(), GameEvent::TravelingEvent(TravelingChoice::new(
        "Castle",
        "You are at the castle. What will you do?",
        "Enter the castle",
        "Keep walking",
        "Indoor",
        "Court",
    )));

    game_events.insert("Court".to_string(), GameEvent::TravelingEvent(TravelingChoice::new(
        "Court",
        "You are in the court. What will you do?",
        "Go to the blacksmith",
        "Leave the castle",
        "Blacksmith",
        "Suspicious_bridge",
    )));

    game_events.insert("Suspicious_bridge".to_string(), GameEvent::CombatEvent(Creature::new(
        "Phantom", 
        50, 
        15, 
        3,
    )));

    let mut player = Creature::new("Player", 100, 10, 5);

    loop {
        if let Some(event) = game_events.get(&game_state.current_location) {
            match event {
                GameEvent::TravelingEvent(traveling_choice) => {
                    traveling_choice.display();

                    println!("Enter your choice (number): ");
                    let mut choice = String::new();
                    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
                    let choice: usize = choice.trim().parse().expect("Please enter a number");

                    match choice {
                        1 => game_state.transition(&traveling_choice.outcome1),
                        2 => game_state.transition(&traveling_choice.outcome2),
                        3 => break,
                        _ => println!("Invalid choice, please enter 1 or 2."),
                    }
                },
                GameEvent::CombatEvent(creature) => {
                    let mut enemy = creature.clone();
                    player.fight(&mut enemy);

                    game_state.transition("Start"); 
                },
            }
        } else {
            println!("You wander into unknown territory.");
            break;
        }

        println!("You are now at: {}", game_state.current_location);
    }
}