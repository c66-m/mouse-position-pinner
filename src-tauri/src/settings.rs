//! 设置读写：`%APPDATA%\MousePositionPinner\settings.json`（Windows）等平台配置目录。
//!
//! 采用「写临时文件 + 原子重命名」避免崩溃/断电导致配置文件损坏。

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WindowPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub hotkey: String,
    pub always_on_top: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<WindowPos>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            hotkey: "F2".into(),
            always_on_top: true,
            window: None,
        }
    }
}

/// Windows: %APPDATA%\MousePositionPinner；macOS: ~/Library/Application Support/MousePositionPinner；
/// Linux: $XDG_CONFIG_HOME/MousePositionPinner（默认 ~/.config/MousePositionPinner）。
pub fn settings_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let base = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
        Path::new(&base).join("MousePositionPinner")
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        Path::new(&home).join("Library/Application Support/MousePositionPinner")
    }
    #[cfg(target_os = "linux")]
    {
        let base = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            format!("{home}/.config")
        });
        Path::new(&base).join("MousePositionPinner")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(".")
    }
}

pub fn settings_path() -> PathBuf {
    settings_dir().join("settings.json")
}

pub fn load() -> Settings {
    load_from(&settings_path())
}

pub fn save(settings: &Settings) -> Result<(), String> {
    save_to(settings, &settings_path())
}

pub fn load_from(path: &Path) -> Settings {
    match std::fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

pub fn save_to(settings: &Settings, path: &Path) -> Result<(), String> {
    let dir = path.parent().ok_or("配置路径无父目录")?;
    std::fs::create_dir_all(dir).map_err(|e| format!("创建配置目录失败：{e}"))?;
    let text = serde_json::to_string_pretty(settings).map_err(|e| format!("序列化设置失败：{e}"))?;
    let tmp = dir.join(".settings.json.tmp");
    std::fs::write(&tmp, text).map_err(|e| format!("写入设置失败：{e}"))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("保存设置失败：{e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path(tag: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("mpp-test-{}-{tag}", std::process::id()));
        p
    }

    #[test]
    fn defaults_are_sane() {
        let s = Settings::default();
        assert_eq!(s.hotkey, "F2");
        assert!(s.always_on_top);
        assert_eq!(s.window, None);
    }

    #[test]
    fn roundtrip_via_file() {
        let path = temp_path("roundtrip.json");
        let _ = std::fs::remove_file(&path);

        let s = Settings {
            hotkey: "Ctrl+Shift+C".into(),
            always_on_top: false,
            window: Some(WindowPos { x: -1920, y: 40 }),
        };
        save_to(&s, &path).unwrap();
        let loaded = load_from(&path);
        assert_eq!(loaded, s);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn missing_file_falls_back_to_default() {
        let path = temp_path("missing.json");
        let _ = std::fs::remove_file(&path);
        assert_eq!(load_from(&path), Settings::default());
    }

    #[test]
    fn corrupt_file_falls_back_to_default() {
        let path = temp_path("corrupt.json");
        std::fs::write(&path, "{ not json !!!").unwrap();
        assert_eq!(load_from(&path), Settings::default());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn parses_camel_case_json() {
        let json = r#"{"hotkey":"F5","alwaysOnTop":false,"window":{"x":100,"y":200}}"#;
        let s: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(s.hotkey, "F5");
        assert!(!s.always_on_top);
        assert_eq!(s.window, Some(WindowPos { x: 100, y: 200 }));
    }

    #[test]
    fn partial_json_uses_defaults_for_missing_fields() {
        let json = r#"{"hotkey":"F6"}"#;
        let s: Settings = serde_json::from_str(json).unwrap();
        assert_eq!(s.hotkey, "F6");
        assert!(s.always_on_top);
        assert_eq!(s.window, None);
    }
}
