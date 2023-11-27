use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use async_std::task::sleep;
use w::{
    co::{HWND_PLACE, LSFW, SW, SWP},
    AllowSetForegroundWindow, AttachThreadInput, GetCurrentThreadId, HwndFocus,
};
use windows::Win32::{
    Foundation::HWND,
    System::Console::{AllocConsole, FreeConsole, GetConsoleWindow},
    UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, SetWindowPos, ASFW_ANY, HWND_TOP, LSFW_LOCK,
        SWP_NOZORDER, WM_SETFOCUS,
    },
};
use winsafe::{self as w, prelude::*};

pub fn run_window(pid: u32) {
    let target_hwnd = Arc::new(Mutex::new(w::HWND::NULL));
    let count = Arc::new(Mutex::new(0));

    let _ = w::EnumWindows(|hwnd| -> bool {
        if hwnd.GetWindowThreadProcessId().1 == pid {
            println!(
                "pid: {}, pid2: {}, window {}",
                pid,
                hwnd.GetWindowThreadProcessId().1,
                hwnd.IsWindowEnabled()
            );
            let mut target_hwnd_lock = target_hwnd.lock().unwrap();
            let mut count_lock = count.lock().unwrap();
            println!(
                "text: {}, bool: {}",
                hwnd.GetWindowText().unwrap(),
                hwnd.IsWindowVisible()
            );
            //if hwnd.GetWindowText().unwrap().contains("Default IME") {
            //active_window(&hwnd);
            //}
            //bring_window(&hwnd);
            if *count_lock == 0 && hwnd.IsWindowVisible() {
                println!("enumtext: {}", hwnd.GetWindowText().unwrap(),);

                //active_window(&hwnd);
                //bring_window(&hwnd);

                *target_hwnd_lock = hwnd;
                *count_lock = 1;
            }
        }

        //

        //check_foreground_window();
        //println!("true {}", hwnd.IsIconic());
        true
    });

    //let default_ime = w::HWND::FindWindowEx(&self, hwnd_child_after, class_name, title)

    let hwnd_lock = target_hwnd.lock().unwrap();

    let hwnd = &*hwnd_lock;
    check_foreground_window();
    println!(
        "setforeground_target_text: {}",
        hwnd.GetWindowText().unwrap()
    );

    active_window(hwnd);
    bring_window(hwnd);

    check_foreground_window();

    let _ = w::EnumWindows(|hwnd| -> bool {
        //check_foreground_window();

        true
    });
}

fn check_foreground_window() {
    match w::HWND::GetForegroundWindow() {
        Some(foreground_hwnd) => match foreground_hwnd.GetWindowText() {
            Ok(text) => {
                println!("title: {}", text)
            }
            Err(_) => {}
        },
        None => {}
    }
}

fn active_window(hwnd: &w::HWND) {
    match w::HWND::GetForegroundWindow() {
        Some(foreground_hwnd) => {
            println!("test {}", foreground_hwnd.GetWindowThreadProcessId().1);
            let result = AttachThreadInput(
                hwnd.GetWindowThreadProcessId().0,
                foreground_hwnd.GetWindowThreadProcessId().0,
                true,
            );
            match result {
                Ok(_) => {
                    println!("atok");
                }
                Err(_) => {
                    println!("atno");
                }
            }

            let result = AllowSetForegroundWindow(Some(ASFW_ANY));
            match result {
                Ok(_) => {
                    println!("tok");
                }
                Err(_) => {
                    println!("no");
                }
            }

            let result = hwnd.SetForegroundWindow();
            match w::HWND::GetForegroundWindow() {
                Some(foreground_hwnd) => match foreground_hwnd.GetWindowText() {
                    Ok(text) => {
                        if hwnd.GetWindowText().unwrap() != text {
                            println!("more f:{}, n:{}", text, hwnd.GetWindowText().unwrap());
                            active_window(hwnd);
                        }
                    }
                    Err(_) => {}
                },
                None => {}
            }

            //hwnd.SetFocus();
            //let _ = hwnd.SetActiveWindow();
            println!("result: {}", result);
            let result = AttachThreadInput(
                hwnd.GetWindowThreadProcessId().0,
                foreground_hwnd.GetWindowThreadProcessId().0,
                false,
            );
            match result {
                Ok(_) => {
                    println!("afok");
                }
                Err(_) => {
                    println!("afno");
                }
            }
        }
        None => {}
    }
}

fn bring_window(hwnd: &w::HWND) {
    if hwnd.IsIconic() {
        hwnd.ShowWindow(SW::RESTORE);
    }
    let _ = hwnd.SetWindowPos(
        w::HwndPlace::Place(HWND_PLACE::TOPMOST),
        w::POINT { x: 0, y: 0 },
        w::SIZE { cx: 800, cy: 900 },
        SWP::NOSIZE | SWP::NOMOVE,
    );
    let _ = hwnd.SetWindowPos(
        w::HwndPlace::Place(HWND_PLACE::NOTOPMOST),
        w::POINT { x: 0, y: 0 },
        w::SIZE { cx: 800, cy: 900 },
        SWP::NOSIZE | SWP::NOMOVE,
    );
}

// unsafe extern "system" fn enum_window(hwnd: HWND, _: LPARAM) -> BOOL {
//     if !is_visiable_window(hwnd) {
//         return TRUE;
//     }

//     TRUE
// }

fn is_visiable_window(hwnd: HWND) -> bool {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        !text.is_empty()
    }
}

fn focus_hwnd(hwnd: HWND) {}

// fn get_exe_from_hwnd(hwnd: HWND) -> Option<String> {
//     unsafe {
//         let mut process_id = 0;
//         GetWindowThreadProcessId(hwnd, Some(&mut process_id));

//         let h_process = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, process_id).unwrap();

//         let mut file_name: [u16; 512] = [0; 512];
//         let size = GetModuleFileNameExW(h_process, None, &mut file_name);
//         let file_name = String::from_utf16_lossy(&file_name[..size as usize]);
//         println!("filename: {}", file_name);

//         Some(file_name.to_string())
//     }
// }

pub fn focus_window() {}
