use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use inputbot::KeybdKey;

use crate::function::Function;

pub struct DobuleTapMacro {
    key: KeybdKey,
    function: Function,
}

impl DobuleTapMacro {
    pub fn new(key: KeybdKey, function: Function) -> Self {
        Self {
            key: key,
            function: function,
        }
    }

    pub fn bind(&self) {
        // example test code
        let timestamp = Arc::new(Mutex::new(Instant::now()));
        let process = Arc::new(Mutex::new(0u8));
        let arc_function = Arc::new(self.function.clone());

        self.key.bind(move || {
            let mut process_lock = process.lock().unwrap();
            let mut timestamp_lock = timestamp.lock().unwrap();

            if timestamp_lock.elapsed().as_secs() >= 1 {
                *process_lock = 0;
            }

            if *process_lock == 0 {
                *process_lock = 1;
                *timestamp_lock = Instant::now();
            } else if *process_lock == 1 {
                *process_lock = 0;

                arc_function.perform();
            }
        });
    }
}
