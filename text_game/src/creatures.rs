#[derive(Clone)]
pub struct Creature {
    pub name: String,
    pub health: u32,
    pub attack: u32,
    pub defense: u32,
}

impl Creature {
    pub fn new(name: &str, health: u32, attack: u32, defense: u32) -> Self {
        Creature {
            name: name.to_string(),
            health,
            attack,
            defense,
        }
    }

    // The fight method remains within the Creature implementation
    pub fn fight(&mut self, other: &mut Creature) {
        if self.attack >= other.defense {
            let attacks_to_kill = other.health / (self.attack - other.defense).max(1);
            let attacks_to_die = self.health / (other.attack - self.defense).max(1); 
            
            if attacks_to_kill <= attacks_to_die {
                println!("{} wins over {} in a deadly battle and learned through the fight.", self.name, other.name);
                self.attack += other.attack / 2;
                self.health = self.health + ((attacks_to_kill - 1) * other.attack) + other.health / 2;
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

pub fn phantom() -> Creature {
    Creature::new("Phantom", 50, 15, 3)
}
