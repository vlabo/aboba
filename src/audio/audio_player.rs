use super::c::*;
use std::path::Path;

use core::ffi::c_void;
use std::ffi::CString;

#[derive(Clone)]
pub struct AudioPlayer {
    inner: *mut c_void,
}

impl AudioPlayer {
    pub fn new() -> Self {
        unsafe {
            AudioPlayer {
                inner: agst_setup(),
            }
        }
    }

    pub fn set_file(&self, path: &Path) {
        let cstring =
            CString::new("file://".to_owned() + path.canonicalize().unwrap().to_str().unwrap())
                .unwrap();
        unsafe { agst_set_file(self.inner, cstring.as_ptr()) };
        self.play();
        self.pause();
    }

    pub fn play(&self) {
        unsafe { agst_play(self.inner) };
    }

    pub fn pause(&self) {
        unsafe { agst_pause(self.inner) };
    }

    pub fn is_playing(&self) -> bool {
        unsafe { agst_is_playing(self.inner) == 1 }
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> i64 {
        unsafe { agst_duration(self.inner) }
    }

    pub fn get_position(&self) -> i64 {
        unsafe { agst_position(self.inner) }
    }

    pub fn set_position(&self, sec: i64) {
        unsafe { agst_seek(self.inner, sec) };
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        unsafe { agst_cleanup(self.inner) };
    }
}
