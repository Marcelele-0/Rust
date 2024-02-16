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

struct Crature {
    name: String,
    health: i32,
    attack: i32,
    defense: i32,
}

impl Crature {
    fn fight(&mut self, other: &mut Crature) {
        let attacks_to_kill = (other.health / (self.attack - other.defense)) as i32;
        let attacks_to_die = (self.health / (other.attack - self.defense)) as i32;
        if attacks_to_kill <= attacks_to_die {
            other.health -= attacks_to_kill * (self.attack - other.defense);
            println!("{} killed {}", self.name, other.name);
        } else {
            self.health -= attacks_to_die * (other.attack - self.defense);
            println!("{} killed {}", other.name, self.name);
        }
    }    
}


struct TravelingEvent {
    location: String,
    description: String,
    choice1: String,
    choice2: String,
    outcome1: String,
    outcome2: String,
}

impl TravelingEvent {
    fn new(location: &str, description: &str, choice1: &str, choice2: &str, outcome1: &str, outcome2: &str) -> Self {
        TravelingEvent {
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


fn main() {
    let mut game_state = GameState::new("Start");

    let start_event = TravelingEvent::new(
        "Start",
        "You are at a crossroad. Which path will you take?",
        "Take the left path to the castle",
        "Take the right path to the forest",
        "Castle",
        "Forest",
    );

    let castle_event = TravelingEvent::new(
        "Castle",
        "You are at the castle. What will you do?",
        "Enter the castle",
        "Keep walking",
        "Indoor",
        "Court",
    );

    let court_event = TravelingEvent::new(
        "Court",
        "You are in the court. What will you do?",
        "Go to the blacksmith",
        "Leave the castle",
        "Blacksmith",
        "Suspicious_bridge",
    );

    let mut player = Crature {
        name: "Player".to_string(),
        health: 100,
        attack: 10,
        defense: 5,
    };

    let mut phantom = Crature {
        name: "Phantom".to_string(),
        health: 50,
        attack: 15,
        defense: 3,
    };


   loop {
    let current_event = match game_state.current_location.as_str() {
        "Start" => &start_event,
        "Castle" => &castle_event,
        "Court" => &court_event,
    
        _ => {
            println!("You are lost!");
            break;
        }
    };
    
    current_event.display();

    println!("Enter your choice (number):");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: usize = choice.trim().parse().expect("Please enter a number");

    
    match choice {
        1 => {
            println!("You chose: {}", current_event.choice1);
            game_state.transition(&current_event.outcome1);
        },
        2 => {
            println!("You chose: {}", current_event.choice2);
            game_state.transition(&current_event.outcome2);
        },
        _ => println!("Invalid choice, please enter 1 or 2."),
    }

    println!("You are now at: {}", game_state.current_location);
   }

   
}