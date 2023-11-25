use inputbot::KeybdKey::*;

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn init(&self) {
        inputbot::handle_input_events();
        // initialize hoykey
    }
}

