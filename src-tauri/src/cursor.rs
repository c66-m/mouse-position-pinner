//! 光标坐标：跨平台获取全局鼠标位置。
//!
//! Windows 下经由 `mouse_position` 内部调用 `GetCursorPos`，
//! 返回的是**虚拟屏（虚拟桌面）坐标**——多显示器布局下主屏左上角为 (0,0)，
//! 左侧/上方扩展的副屏坐标为负值。这与 ShadowClicker 脚本读取光标的口径一致。

pub fn get_cursor_pos() -> Result<(i32, i32), String> {
    use mouse_position::mouse_position::Mouse;
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Ok((x, y)),
        Mouse::Error => Err("获取鼠标位置失败".into()),
    }
}

/// 复制到剪贴板的纯坐标文本，如 `1234, 567`（与前端 formatCoords 保持一致）。
pub fn format_coords(x: i32, y: i32) -> String {
    format!("{}, {}", x, y)
}

#[cfg(test)]
mod tests {
    use super::format_coords;

    #[test]
    fn formats_positive_coords() {
        assert_eq!(format_coords(1234, 567), "1234, 567");
    }

    #[test]
    fn formats_negative_coords() {
        assert_eq!(format_coords(-1920, 300), "-1920, 300");
    }

    #[test]
    fn formats_zero() {
        assert_eq!(format_coords(0, 0), "0, 0");
    }
}
