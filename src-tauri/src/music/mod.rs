use inputbot::KeybdKey;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::sleep;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow};

// 将 Rust 字符串 (&str) 转换为 Windows API 使用的宽字符（UTF-16）空结尾字符串
fn to_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

// 添加一个包装类型来使 HWND 可以安全地在线程间传递
#[derive(Clone, Copy)]
struct SafeHWND(HWND);
unsafe impl Send for SafeHWND {}
unsafe impl Sync for SafeHWND {}

// 添加全局窗口句柄缓存
static WINDOW_HANDLE: Lazy<Mutex<Option<SafeHWND>>> = Lazy::new(|| Mutex::new(None));

fn get_window_handle() -> Option<HWND> {
    let mut handle = WINDOW_HANDLE.lock().ok()?;
    if handle.is_none() {
        let window_class_name = to_wide_string("TgcMainWindow");
        let hwnd: HWND = unsafe { FindWindowW(window_class_name.as_ptr(), std::ptr::null()) };
        if hwnd != std::ptr::null_mut() {
            *handle = Some(SafeHWND(hwnd));
        }
    }
    handle.map(|h| h.0)
}

pub async fn press_key(key: &str) {
    // 获取缓存的窗口句柄
    let hwnd = match get_window_handle() {
        Some(handle) => handle,
        None => {
            println!("找不到目标窗口");
            return;
        }
    };

    // 将窗口置于前台并激活
    unsafe {
        SetForegroundWindow(hwnd);
    }

    // 解析按键
    let key = parse_note(key);
    key.press();
    sleep(Duration::from_millis(100)).await;
    key.release();
}

pub fn parse_note(key: &str) -> KeybdKey {
    // 以Key为分隔符，分割字符串
    let parts: Vec<&str> = key.split("Key").collect();
    let key = parts[1];
    match key {
        "0" => KeybdKey::YKey,
        "1" => KeybdKey::UKey,
        "2" => KeybdKey::IKey,
        "3" => KeybdKey::OKey,
        "4" => KeybdKey::PKey,
        "5" => KeybdKey::HKey,
        "6" => KeybdKey::JKey,
        "7" => KeybdKey::KKey,
        "8" => KeybdKey::LKey,
        "9" => KeybdKey::SemicolonKey,
        "10" => KeybdKey::NKey,
        "11" => KeybdKey::MKey,
        "12" => KeybdKey::CommaKey,
        "13" => KeybdKey::PeriodKey,
        "14" => KeybdKey::SlashKey,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note() {
        assert_eq!(parse_note("Key0"), KeybdKey::YKey);
        assert_eq!(parse_note("Key1"), KeybdKey::UKey);
        assert_eq!(parse_note("Key2"), KeybdKey::IKey);
        assert_eq!(parse_note("Key3"), KeybdKey::OKey);
        assert_eq!(parse_note("Key4"), KeybdKey::PKey);
        assert_eq!(parse_note("Key5"), KeybdKey::HKey);
        assert_eq!(parse_note("Key6"), KeybdKey::JKey);
        assert_eq!(parse_note("Key7"), KeybdKey::KKey);
        assert_eq!(parse_note("Key8"), KeybdKey::LKey);
        assert_eq!(parse_note("Key9"), KeybdKey::SemicolonKey);
        assert_eq!(parse_note("Key10"), KeybdKey::NKey);
        assert_eq!(parse_note("Key11"), KeybdKey::MKey);
        assert_eq!(parse_note("Key12"), KeybdKey::CommaKey);
        assert_eq!(parse_note("Key13"), KeybdKey::PeriodKey);
        assert_eq!(parse_note("Key14"), KeybdKey::SlashKey);
    }

    #[tokio::test]
    async fn test_press_key() {
        press_key("Key1").await;
        press_key("Key2").await;
        press_key("Key3").await;
        press_key("Key4").await;
        press_key("Key5").await;
        press_key("Key6").await;
        press_key("Key7").await;
        press_key("Key8").await;
        press_key("Key9").await;
    }
}
