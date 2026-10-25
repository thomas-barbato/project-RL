//! Read-only Win32 cursor snapshot. No input, focus or system-setting changes.
use std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }
#[repr(C)]
struct CursorInfo { size: u32, flags: u32, cursor: *mut c_void, position: Point }

#[link(name = "user32")]
unsafe extern "system" {
    fn GetCursorInfo(info: *mut CursorInfo) -> i32;
    fn WindowFromPoint(point: Point) -> *mut c_void;
    fn GetWindowThreadProcessId(window: *mut c_void, process: *mut u32) -> u32;
    fn GetClassLongPtrW(window: *mut c_void, index: i32) -> usize;
    fn LoadCursorW(instance: *mut c_void, name: *const u16) -> *mut c_void;
}

fn main() {
    unsafe {
        let mut info = CursorInfo { size: std::mem::size_of::<CursorInfo>() as u32,
            flags: 0, cursor: std::ptr::null_mut(), position: Point { x: 0, y: 0 } };
        assert_ne!(GetCursorInfo(&mut info), 0, "{}", std::io::Error::last_os_error());
        let window = WindowFromPoint(info.position);
        let mut process = 0;
        GetWindowThreadProcessId(window, &mut process);
        let arrow = LoadCursorW(std::ptr::null_mut(), 32512usize as *const u16);
        let hand = LoadCursorW(std::ptr::null_mut(), 32649usize as *const u16);
        let cross = LoadCursorW(std::ptr::null_mut(), 32515usize as *const u16);
        println!("{{\"flags\":{},\"visible\":{},\"cursor_handle\":{},\"window_under_pointer\":{},\"process_under_pointer\":{},\"class_cursor_handle\":{},\"arrow_handle\":{},\"hand_handle\":{},\"cross_handle\":{},\"position\":[{},{}]}}",
            info.flags, info.flags & 1 != 0, info.cursor as usize, window as usize,
            process, GetClassLongPtrW(window, -12), arrow as usize, hand as usize,
            cross as usize, info.position.x, info.position.y);
    }
}
