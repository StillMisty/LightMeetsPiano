use inputbot::KeybdKey;
use std::time::Duration;
use tokio::time::sleep;

#[cfg(target_os = "windows")]
mod windows;

pub async fn press_key(key: &str) {
    #[cfg(target_os = "windows")]
    {
        // 将窗口置于前台并激活
        windows::activate_window();
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
