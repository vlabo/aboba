use core::ffi::c_void;
use std::os::raw::c_char;

extern "C" {
    pub fn agst_setup() -> *mut c_void;
    pub fn agst_cleanup(player: *mut c_void);
    pub fn agst_set_file(player: *mut c_void, uri: *const c_char);

    pub fn agst_play(player: *mut c_void) -> i32;
    pub fn agst_pause(player: *mut c_void) -> i32;
    pub fn agst_is_playing(player: *mut c_void) -> i32;

    pub fn agst_position(player: *mut c_void) -> i64;
    pub fn agst_duration(player: *mut c_void) -> i64;
    pub fn agst_seek(player: *mut c_void, sec: i64);
}
