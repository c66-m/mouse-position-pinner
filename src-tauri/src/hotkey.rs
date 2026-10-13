//! 热键字符串解析：把 "F2"、"Ctrl+Shift+A" 等转成全局快捷键插件的 Shortcut。
//! 支持修饰键 Ctrl / Alt / Shift / Super(Win/Cmd)，分隔符为 `+`，大小写与空格不敏感。

use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub fn parse_hotkey(input: &str) -> Result<Shortcut, String> {
    let mut modifiers = Modifiers::empty();
    let mut key: Option<&str> = None;

    for token in input.split(|c: char| c == '+' || c.is_whitespace()) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        match token.to_ascii_lowercase().as_str() {
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            "super" | "meta" | "win" | "cmd" => modifiers |= Modifiers::SUPER,
            _ => {
                if key.is_some() {
                    return Err(format!("无效热键：{input}"));
                }
                key = Some(token);
            }
        }
    }

    let key = key.ok_or_else(|| format!("无效热键：{input}"))?;
    let code = key_code(key)?;
    Ok(Shortcut::new(Some(modifiers), code))
}

fn key_code(key: &str) -> Result<Code, String> {
    use Code::*;
    let k = key.to_ascii_lowercase();
    let code = match k.as_str() {
        "f1" => F1,
        "f2" => F2,
        "f3" => F3,
        "f4" => F4,
        "f5" => F5,
        "f6" => F6,
        "f7" => F7,
        "f8" => F8,
        "f9" => F9,
        "f10" => F10,
        "f11" => F11,
        "f12" => F12,
        "f13" => F13,
        "f14" => F14,
        "f15" => F15,
        "f16" => F16,
        "f17" => F17,
        "f18" => F18,
        "f19" => F19,
        "f20" => F20,
        "f21" => F21,
        "f22" => F22,
        "f23" => F23,
        "f24" => F24,
        "a" => KeyA,
        "b" => KeyB,
        "c" => KeyC,
        "d" => KeyD,
        "e" => KeyE,
        "f" => KeyF,
        "g" => KeyG,
        "h" => KeyH,
        "i" => KeyI,
        "j" => KeyJ,
        "k" => KeyK,
        "l" => KeyL,
        "m" => KeyM,
        "n" => KeyN,
        "o" => KeyO,
        "p" => KeyP,
        "q" => KeyQ,
        "r" => KeyR,
        "s" => KeyS,
        "t" => KeyT,
        "u" => KeyU,
        "v" => KeyV,
        "w" => KeyW,
        "x" => KeyX,
        "y" => KeyY,
        "z" => KeyZ,
        "0" => Digit0,
        "1" => Digit1,
        "2" => Digit2,
        "3" => Digit3,
        "4" => Digit4,
        "5" => Digit5,
        "6" => Digit6,
        "7" => Digit7,
        "8" => Digit8,
        "9" => Digit9,
        "space" => Space,
        "enter" | "return" => Enter,
        "esc" | "escape" => Escape,
        "tab" => Tab,
        "backspace" => Backspace,
        "delete" | "del" => Delete,
        "insert" | "ins" => Insert,
        "home" => Home,
        "end" => End,
        "pageup" | "pgup" => PageUp,
        "pagedown" | "pgdn" => PageDown,
        "up" | "arrowup" => ArrowUp,
        "down" | "arrowdown" => ArrowDown,
        "left" | "arrowleft" => ArrowLeft,
        "right" | "arrowright" => ArrowRight,
        "capslock" => CapsLock,
        "printscreen" | "prtsc" => PrintScreen,
        "scrolllock" => ScrollLock,
        "pause" => Pause,
        "numlock" => NumLock,
        _ => return Err(format!("不支持的按键：{key}")),
    };
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_f2() {
        let hk = parse_hotkey("F2").unwrap();
        assert_eq!(hk.key, Code::F2);
        assert_eq!(hk.mods, Modifiers::empty());
    }

    #[test]
    fn parses_modifier_combination() {
        let hk = parse_hotkey("Ctrl+Shift+A").unwrap();
        assert_eq!(hk.key, Code::KeyA);
        assert_eq!(
            hk.mods,
            Modifiers::CONTROL | Modifiers::SHIFT
        );
    }

    #[test]
    fn ignores_case_and_whitespace() {
        let hk = parse_hotkey(" ctrl + f1 ").unwrap();
        assert_eq!(hk.key, Code::F1);
        assert_eq!(hk.mods, Modifiers::CONTROL);
    }

    #[test]
    fn parses_win_alt_digit() {
        let hk = parse_hotkey("win+alt+9").unwrap();
        assert_eq!(hk.key, Code::Digit9);
        assert_eq!(hk.mods, Modifiers::SUPER | Modifiers::ALT);
    }

    #[test]
    fn parses_f13() {
        let hk = parse_hotkey("F13").unwrap();
        assert_eq!(hk.key, Code::F13);
    }

    #[test]
    fn rejects_empty() {
        assert!(parse_hotkey("").is_err());
        assert!(parse_hotkey("   ").is_err());
    }

    #[test]
    fn rejects_duplicate_key() {
        assert!(parse_hotkey("Ctrl+Ctrl").is_err());
        assert!(parse_hotkey("A+B").is_err());
    }

    #[test]
    fn rejects_unknown_key() {
        assert!(parse_hotkey("Ctrl+张").is_err());
    }
}
