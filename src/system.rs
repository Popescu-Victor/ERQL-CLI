use windows::Win32::Foundation::{BOOL, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible,
};
use windows::core::PWSTR;

// Returnign list of open windows.


pub fn get_open_window_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    unsafe {
        // Pass a mutable pointer to our Vec through LPARAM so the callback can fill it
        let param = LPARAM(&mut names as *mut Vec<String> as isize);
        let _ = EnumWindows(Some(enum_windows_callback), param);
    }

    names
}

unsafe extern "system" fn enum_windows_callback(hwnd: windows::Win32::Foundation::HWND, lparam: LPARAM) -> BOOL {
    let names = &mut *(lparam.0 as *mut Vec<String>);

    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }

    let len = GetWindowTextLengthW(hwnd);
    if len > 0 {
        let mut buf: Vec<u16> = vec![0; (len + 1) as usize];
        let copied = GetWindowTextW(hwnd, &mut buf);
        if copied > 0 {
            let title = String::from_utf16_lossy(&buf[..copied as usize]);
            if !title.trim().is_empty() {
                names.push(title);
            }
        }
    }

    BOOL(1)
}
