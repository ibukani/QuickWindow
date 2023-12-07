use async_std::task::JoinHandle;
use inputbot::KeybdKey::*;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use crate::function::{feature::Feature, Function};

use self::double_tap_macro::DobuleTapMacro;

pub mod double_tap_macro;

pub struct InputManager {
    read_task: Option<JoinHandle<bool>>,
}

impl InputManager {
    pub fn new() -> Self {
        return Self { read_task: None };
    }

    pub fn init(&mut self) {
        self.read_task = Some(async_std::task::spawn(async {
            DobuleTapMacro::new(LShiftKey, Function::new(Feature::Application {
                file_path: "C:\\Program Files\\WindowsApps\\Microsoft.WindowsTerminal_1.18.2822.0_x64__8wekyb3d8bbwe\\WindowsTerminal.exe",
            }));
            DobuleTapMacro::new(
                LControlKey,
                Function::new(Feature::Application {
                    file_path: "C:\\Program Files\\Notepad++\\notepad++.exe",
                }),
            );

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
