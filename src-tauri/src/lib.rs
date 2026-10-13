//! 鼠标位置定位器 主逻辑：窗口装配、IPC 命令、全局热键、剪贴板与位置记忆。

mod cursor;
mod hotkey;
mod settings;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{Emitter, Manager, PhysicalPosition, WindowEvent};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const WIN_W: i32 = 190;
const WIN_H: i32 = 64;
const SCREEN_MARGIN: i32 = 24;

pub struct AppState {
    settings: Mutex<settings::Settings>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            settings: Mutex::new(settings::Settings::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MousePos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyPayload {
    pub x: i32,
    pub y: i32,
    pub text: String,
}

// ---------- 命令 ----------

#[tauri::command]
fn get_cursor_pos() -> Result<MousePos, String> {
    let (x, y) = cursor::get_cursor_pos()?;
    Ok(MousePos { x, y })
}

/// 读取当前光标坐标 → 写入剪贴板（纯坐标文本）→ 向窗口广播 copy-done。
fn copy_current_pos(app: &tauri::AppHandle) -> Result<(), String> {
    let (x, y) = cursor::get_cursor_pos()?;
    let text = cursor::format_coords(x, y);
    app.clipboard()
        .write_text(text.clone())
        .map_err(|e| format!("写入剪贴板失败：{e}"))?;
    app.emit("copy-done", CopyPayload { x, y, text })
        .map_err(|e| format!("通知前端失败：{e}"))?;
    Ok(())
}

#[tauri::command]
fn copy_cursor_pos(app: tauri::AppHandle) -> Result<(), String> {
    copy_current_pos(&app)
}

#[tauri::command]
fn get_settings(app: tauri::AppHandle) -> Result<settings::Settings, String> {
    app.state::<AppState>()
        .settings
        .lock()
        .map(|s| s.clone())
        .map_err(|e| e.to_string())
}

/// 更新热键 / 置顶开关并持久化；热键变化时注销旧键、注册新键。
/// 窗口位置由 Rust 侧 Moved 事件自动记忆，patch 里的 window 字段会被忽略。
#[tauri::command]
fn update_settings(
    app: tauri::AppHandle,
    patch: settings::Settings,
) -> Result<settings::Settings, String> {
    let state = app.state::<AppState>();
    let mut cur = state.settings.lock().map_err(|e| e.to_string())?;
    let old = cur.clone();

    // 校验并规范化热键（无效则直接报错，不改动现有设置）
    let new_hotkey = hotkey::parse_hotkey(&patch.hotkey)?;

    if old.hotkey != patch.hotkey {
        if let Ok(old_hk) = hotkey::parse_hotkey(&old.hotkey) {
            let _ = app.global_shortcut().unregister(old_hk);
        }
        app.global_shortcut()
            .register(new_hotkey)
            .map_err(|e| format!("热键注册失败（可能已被其他程序占用）：{e}"))?;
    }

    if old.always_on_top != patch.always_on_top {
        if let Some(win) = app.get_webview_window("main") {
            win.set_always_on_top(patch.always_on_top)
                .map_err(|e| format!("设置窗口置顶失败：{e}"))?;
        }
    }

    cur.hotkey = patch.hotkey;
    cur.always_on_top = patch.always_on_top;
    settings::save(&cur)?;
    Ok(cur.clone())
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

// ---------- 窗口位置 ----------

/// 恢复位置时确保窗口仍落在某个显示器内；否则回退到主屏右上角。
fn clamp_to_monitor(app: &tauri::AppHandle, pos: PhysicalPosition<i32>) -> PhysicalPosition<i32> {
    let monitors = app.available_monitors().unwrap_or_default();
    let on_screen = monitors.iter().any(|m| {
        let p = m.position();
        let s = m.size();
        pos.x >= p.x
            && pos.x + WIN_W <= p.x + s.width as i32
            && pos.y >= p.y
            && pos.y + WIN_H <= p.y + s.height as i32
    });
    if on_screen {
        return pos;
    }
    top_right_primary(app)
}

fn top_right_primary(app: &tauri::AppHandle) -> PhysicalPosition<i32> {
    if let Some(m) = app.primary_monitor().ok().flatten() {
        let p = m.position();
        let s = m.size();
        PhysicalPosition::new(p.x + s.width as i32 - WIN_W - SCREEN_MARGIN, p.y + SCREEN_MARGIN)
    } else {
        PhysicalPosition::new(100, 100)
    }
}

fn apply_position(app: &tauri::AppHandle, s: &settings::Settings) {
    let pos = match s.window {
        Some(w) => clamp_to_monitor(app, PhysicalPosition::new(w.x, w.y)),
        None => top_right_primary(app),
    };
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_position(pos);
    }
}

fn register_global_hotkey(app: &tauri::AppHandle, hotkey_str: &str) -> Result<(), String> {
    let hk = hotkey::parse_hotkey(hotkey_str)?;
    app.global_shortcut()
        .register(hk)
        .map_err(|e| format!("热键注册失败：{e}"))?;
    Ok(())
}

// ---------- 装配 ----------

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app_handle, _shortcut, event| {
                    if event.state == ShortcutState::Pressed {
                        if let Err(e) = copy_current_pos(app_handle) {
                            eprintln!("鼠标位置定位器: copy failed: {e}");
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_cursor_pos,
            copy_cursor_pos,
            get_settings,
            update_settings,
            exit_app
        ])
        .setup(|app| {
            let handle = app.handle().clone();

            let loaded = settings::load();
            if let Ok(mut s) = app.state::<AppState>().settings.lock() {
                *s = loaded.clone();
            }

            if let Some(win) = app.get_webview_window("main") {
                let _ = win.set_always_on_top(loaded.always_on_top);
            }

            apply_position(&handle, &loaded);

            if let Err(e) = register_global_hotkey(&handle, &loaded.hotkey) {
                eprintln!("鼠标位置定位器: {e}");
            }

            let (pos_tx, pos_rx) = std::sync::mpsc::channel::<(i32, i32)>();
            let saver = handle.clone();
            std::thread::spawn(move || {
                let mut pending: Option<(i32, i32)> = None;
                let mut last_write = std::time::Instant::now();
                loop {
                    match pos_rx.recv_timeout(Duration::from_millis(150)) {
                        Ok(pos) => pending = Some(pos),
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                    if last_write.elapsed() >= Duration::from_millis(250) {
                        if let Some(pos) = pending.take() {
                            if let Some(state) = saver.try_state::<AppState>() {
                                if let Ok(mut s) = state.settings.lock() {
                                    s.window = Some(settings::WindowPos {
                                        x: pos.0,
                                        y: pos.1,
                                    });
                                    let _ = settings::save(&s);
                                }
                            }
                            last_write = std::time::Instant::now();
                        }
                    }
                }
            });

            if let Some(win) = app.get_webview_window("main") {
                win.on_window_event(move |event| {
                    if let WindowEvent::Moved(pos) = event {
                        let _ = pos_tx.send((pos.x, pos.y));
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("鼠标位置定位器 运行失败");
}
