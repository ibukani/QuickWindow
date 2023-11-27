use std::path::Path;

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[cfg(target_os = "windows")]
pub mod application_windows;
#[cfg(target_os = "windows")]
pub use application_windows::*;

pub fn run_application(file_path: &&str) {
    let mut sys = System::new_all();

    sys.refresh_processes();

    match sys.processes().iter().find(|(_k, v)| {
        if !v.exe().is_file() {
            return false;
        }
        println!("bool: {}", v.exe() == Path::new(file_path));

        v.exe() == Path::new(file_path)
    }) {
        // 多重起動防止
        Some((pid, _process)) => {
            switch_window(pid.as_u32());
        }
        // 起動処理
        None => {
            start_window();
        }
    };
}
