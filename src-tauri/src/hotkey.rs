use crate::function;
use crate::function::feature::Feature;
use crate::function::Function;
use async_std::task::JoinHandle;
use inputbot::KeybdKey::*;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct InputManager {
    read_task: Option<JoinHandle<bool>>,
}

impl InputManager {
    pub fn new() -> Self {
        return Self { read_task: None };
    }

    pub fn init(&mut self) {
        self.read_task = Some(async_std::task::spawn(async {
            // example test code
            let timestamp = Arc::new(Mutex::new(Instant::now()));
            let process = Arc::new(Mutex::new(0u8));

            RShiftKey.bind(move || {
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

                    Function::new(Feature::Application {
                        file_path: //"C:\\Users\\ibueb\\Projects\\tauri\\QuickWindow\\src-tauri\\target\\debug\\QuickWindow.exe"
                        //"C:\\Program Files\\Notepad++\\notepad++.exe"
                        "C:\\Program Files\\WindowsApps\\Microsoft.WindowsTerminal_1.18.2822.0_x64__8wekyb3d8bbwe\\WindowsTerminal.exe",
                    })
                    .perform();
                    println!("double tap!")
                }
            });

            inputbot::handle_input_events();
            true
        }));
    }
    //
    // pub fn exit(self) {
    //     match self.read_task {
    //         None => {}
    //         Some(task) => {
    //             block_on(task)
    //         }
    //     }
    // }
}
