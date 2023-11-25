use inputbot::handle_input_events;

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn init(&self) {        

        handle_input_events();
        // initialize hoykey
    }
}
