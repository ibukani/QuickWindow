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
        let tap_timestamp = Arc::new(Mutex::new(Instant::now()));
        let ct_timestamp = Arc::new(Mutex::new(Instant::now()));
        let process = Arc::new(Mutex::new(0u8));
        let arc_function = Arc::new(self.function.clone());

        self.key.bind(move || {
            let mut process_lock = process.lock().unwrap();
            let mut tap_timestamp_lock = tap_timestamp.lock().unwrap();
            let mut ct_timestamp_lock = ct_timestamp.lock().unwrap();

            if *process_lock == 2 && ct_timestamp_lock.elapsed().as_secs() >= 2 {
                *process_lock = 0;
            }

            if *process_lock != 2 && tap_timestamp_lock.elapsed().as_secs() >= 1 {
                *process_lock = 0;
            }

            if *process_lock == 0 {
                *process_lock = 1;
                *tap_timestamp_lock = Instant::now();
            } else if *process_lock == 1 {
                *process_lock = 2;

                arc_function.perform();
                *ct_timestamp_lock = Instant::now();
            }
        });
    }
}
