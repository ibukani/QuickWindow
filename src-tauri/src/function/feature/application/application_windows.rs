use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use w::co::{HWND_PLACE, SW, SWP};
use winsafe::{self as w, prelude::*};

pub fn start_window(exe_file_path: &&str) {
    println!("{}", exe_file_path);
    Command::new("cmd")
        .args(["/C", "start", "", exe_file_path])
        .status()
        .expect("failed to execute start");
}

pub fn switch_window(pid: u32) {
    let target_hwnd = Arc::new(Mutex::new(w::HWND::NULL));

    // pidからメインのウィンドウを取得する
    let _ = w::EnumWindows(|hwnd| -> bool {
        if hwnd.GetWindowThreadProcessId().1 == pid && hwnd.IsWindowVisible() {
            let mut target_hwnd_lock = target_hwnd.lock().unwrap();
            *target_hwnd_lock = hwnd;
        }
        true
    });

    // データの加工
    let hwnd_lock = target_hwnd.lock().unwrap();
    let hwnd = &*hwnd_lock;

    // 選択状態に変更
    focus_window(hwnd);
    // トップ画面へ
    // focus_windowで処理が完結する
    //bring_window_to_top(hwnd);
}

/// [`w::HWND`]にフォーカスさせる
fn focus_window(hwnd: &w::HWND) {
    if let Some(foreground_hwnd) = w::HWND::GetForegroundWindow() {
        let _ = w::AttachThreadInput(
            hwnd.GetWindowThreadProcessId().0,
            foreground_hwnd.GetWindowThreadProcessId().0,
            true,
        );

        //let _ = AllowSetForegroundWindow(Some(ASFW_ANY));

        hwnd.SetForegroundWindow();
        // 設定できていない場合は再帰的に行う
        if let Some(changed_foreground_hwnd) = w::HWND::GetForegroundWindow() {
            if let Ok(text) = changed_foreground_hwnd.GetWindowText() {
                if hwnd.GetWindowText().unwrap() != text {
                    focus_window(hwnd);
                }
            }
        }

        let _ = w::AttachThreadInput(
            hwnd.GetWindowThreadProcessId().0,
            foreground_hwnd.GetWindowThreadProcessId().0,
            false,
        );
    }
}

/// 選択したWindowをTopに持ってくる
/// 必要なさそう
fn _bring_window_to_top(hwnd: &w::HWND) {
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
