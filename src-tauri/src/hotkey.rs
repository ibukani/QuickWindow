use inputbot::KeybdKey::*;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct InputManager {}

impl InputManager {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn init(&self) {
        // example test code
        let mut timestamp = Arc::new(Mutex::new(Instant::now()));
        let mut process = Arc::new(Mutex::new(0 as u8));

        RShiftKey.bind(move || {
            let mut process_lock = process.lock().unwrap();
            let mut timestamp_lock = timestamp.lock().unwrap();

            if *process_lock == 0 {
                *process_lock = 1;
                *timestamp_lock = Instant::now();
            } else if *process_lock == 1 {
                *process_lock = 0;

                if timestamp_lock.elapsed().as_secs() < 1 {
                    println!("double tap!");
                }
            }
        });

        inputbot::handle_input_events();
    }
}
