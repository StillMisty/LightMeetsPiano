use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, SetForegroundWindow};
use once_cell::sync::Lazy;
use std::sync::Mutex;

// 定义窗口激活的trait
pub trait WindowActivation {
    fn activate(&self);
}

// 添加一个包装类型来使 HWND 可以安全地在线程间传递
#[derive(Clone, Copy)]
struct SafeHWND(HWND);
unsafe impl Send for SafeHWND {}
unsafe impl Sync for SafeHWND {}

// Windows窗口管理器
pub struct WindowManager {
    window_class_name: String,
}

impl WindowManager {
    pub fn new(window_class_name: &str) -> Self {
        Self {
            window_class_name: window_class_name.to_string(),
        }
    }

    // 将 Rust 字符串 (&str) 转换为 Windows API 使用的宽字符（UTF-16）空结尾字符串
    fn to_wide_string(&self, s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    fn get_window_handle(&self) -> Option<HWND> {
        let window_class_name = self.to_wide_string(&self.window_class_name);
        let hwnd: HWND = unsafe { FindWindowW(window_class_name.as_ptr(), std::ptr::null()) };
        if hwnd != std::ptr::null_mut() {
            Some(hwnd)
        } else {
            None
        }
    }
}

impl WindowActivation for WindowManager {
    fn activate(&self) {
        // 获取窗口句柄
        let hwnd = match self.get_window_handle() {
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
    }
}

// 全局窗口管理器实例
static WINDOW_MANAGER: Lazy<WindowManager> = Lazy::new(|| WindowManager::new("TgcMainWindow"));

// 公共接口
pub(crate) fn activate_window() {
    WINDOW_MANAGER.activate();
}