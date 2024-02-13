    // Structs get camel case.
    // Variables get snake case.
    // Constants get all upper case.


use std::io;

struct GameState {
    current_location: String,
}

impl GameState {
    fn update_location(&mut self, new_location: String) {
        self.current_location = new_location;
    }

    fn transition(&mut self, outcome: String) {
        self.update_location(outcome);
    }
}

struct RandomEvent {
    event_name: String,
    event_description: String,
    choices: Vec<String>,
    outcomes: Vec<String>,
}

impl RandomEvent {

    fn display(&self) {
        println!("Event: {}", self.event_name);
        println!("{}", self.event_description);
        for (i, choice) in self.choices.iter().enumerate() {
            println!("{}: {}", i + 1, choice);
        }
    }

    fn handle_choice(&self, choice_index: usize) -> String {
        if choice_index < self.outcomes.len() {
            self.outcomes[choice_index].clone()
        } else {
            "Invalid choice".to_string()
        }
    }
}
    


fn main() {
    let mut game_state = GameState {
        current_location: "The Start".to_string(),
    };

    // Assuming you have a function to get the current event based on game_state
    let event = get_current_event(&game_state);

    event.display();

    let mut choice = String::new();
    println!("Enter your choice (number):");
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: usize = choice.trim().parse().expect("Please type a number!") - 1;

    let outcome = event.handle_choice(choice);
    game_state.transition(outcome);

    println!("You are now at: {}", game_state.current_location);

    // Continue with the next part of the loop or end the game based on the outcome
}
