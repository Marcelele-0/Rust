//main.rs

use std::collections::HashMap;

mod creatures;
mod events;
mod game_state;

use crate::creatures::Creature;
use crate::game_state::GameState;
use crate::events::TravelingDualChoice;
use crate::events::GameEvent;

fn main() {
    let mut game_events: HashMap<String, GameEvent> = HashMap::new();
    let mut game_state = GameState::new("Crossroad");

    game_events.insert("Crossroad".to_string(), GameEvent::TravelingDualEvent(TravelingDualChoice::new(
        "Crossroad",
        "You are at a crossroad. Which path will you take?",
        &[
            ("Go to Castle".to_string(),"castle".to_string()),
            ("Go to Forest".to_string(),"forest".to_string())
        ]
    )));

    game_events.insert("Crossroad".to_string(), GameEvent::CombatEvent(Creature::new(
        "Wolf",
        50, 
        5, 
        2, 
        "River")));

    let mut player = Creature::new("Player", 100, 10, 5," ",);

    loop {
        if let Some(event) = game_events.get(&game_state.current_location) {
            match event {
                GameEvent::TravelingDualEvent(traveling_choice) => {
                    traveling_choice.display();

                    println!("Enter your choice (number): ");
                    
                    let mut choice = String::new();
                    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
                    
                    let choice: usize = choice.trim().parse().expect("Please enter a number");

                    match choice {
                        0 => break,
                        _ => {
                        if choice > 0 && choice <= traveling_choice.choices.len() as usize {
                            let choice_index = choice - 1;
                            game_state.transition(&traveling_choice.choices[choice_index].1);
                        } else {
                            println!("Invalid choice, please enter a number between 1 and {}.", traveling_choice.choices.len());
                        }}
                    }
                }
                
                GameEvent::CombatEvent(creature) => {
                    let mut enemy = creature.clone();
                    player.fight(&mut enemy);

                    game_state.transition(&enemy.post_location); 
                }
            }
        } else {
            println!("You wander into unknown territory.");
            break;
        }
        println!("You are now at: {}", game_state.current_location);
    }
}