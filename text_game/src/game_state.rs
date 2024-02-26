pub struct GameState {
    pub current_location: String,
}

impl GameState {
    pub fn new(start_location: &str) -> Self {
        GameState {
            current_location: start_location.to_string(),
        }
    }

    pub fn transition(&mut self, new_location: &str) {
        self.current_location = new_location.to_string();
    }
}
