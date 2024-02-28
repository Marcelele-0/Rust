//main.rs

use std::collections::HashMap;

mod creatures;
mod events;
mod game_state;

use crate::creatures::Creature;
use crate::game_state::GameState;
use crate::events::TravelingDualChoice;
use crate::events::TravelingTripleChoice;
use crate::events::GameEvent;

fn main() {
    let mut game_events: HashMap<String, GameEvent> = HashMap::new();
    let mut game_state = GameState::new("Start");

    game_events.insert("Start".to_string(), GameEvent::TravelingDualEvent(TravelingDualChoice::new(
        "Start",
        "You are at a crossroad. Which path will you take?",
        "Go to Castle",
        "Go to Forest",
        "Castle",
        "Forest",
    )));

    game_events.insert("Castle".to_string(), GameEvent::TravelingDualEvent(TravelingDualChoice::new(
        "Castle",
        "You are at the castle. What will you do?",
        "Enter the castle",
        "Keep walking",
        "Indoor",
        "Court",
    )));

    game_events.insert("Court".to_string(), GameEvent::TravelingDualEvent(TravelingDualChoice::new(
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
        "Bridge_ruins",
    )));

    game_events.insert("Bridge_ruins".to_string(), GameEvent::TravelingTripleEvent(TravelingTripleChoice::new(
        "South of the bridge ruins",
        "You are at the south of the bridge ruins. What will you do?",
        "go back",
        "Go up the river",
        "follow the path",
        "Castle",
        "River",
        "south path",

    )));

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
                        1 => game_state.transition(&traveling_choice.outcome1),
                        2 => game_state.transition(&traveling_choice.outcome2),
                        3 => break,
                        _ => println!("Invalid choice, please enter 1 or 2."),
                    }
                },
                GameEvent::CombatEvent(creature) => {
                    let mut enemy = creature.clone();
                    player.fight(&mut enemy);

                    game_state.transition(&enemy.post_location); 
                },
                GameEvent::TravelingTripleEvent(traveling_choice) => {
                    traveling_choice.display();

                    println!("Enter your choice (number): ");
                    let mut choice = String::new();
                    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
                    let choice: usize = choice.trim().parse().expect("Please enter a number");

                    match choice {
                        1 => game_state.transition(&traveling_choice.outcome1),
                        2 => game_state.transition(&traveling_choice.outcome2),
                        3 => game_state.transition(&traveling_choice.outcome3),
                        4 => break,
                        _ => println!("Invalid choice, please enter 1, 2, or 3."),
                    }
                },
            }
        } else {
            println!("You wander into unknown territory.");
            break;
        }

        println!("You are now at: {}", game_state.current_location);
    }
}