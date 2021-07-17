use std::path::Path;

use core::ffi::c_void;
use std::ffi::CString;

mod gstreamer;

#[derive(Clone)]
pub struct Player {
    inner: *mut c_void,
}

impl Player {
    pub fn new() -> Self {
        unsafe {
            Player {
                inner: gstreamer::agst_setup(),
            }
        }
    }

    pub fn set_file(&self, path: &Path) {
        let cstring =
            CString::new("file://".to_owned() + path.canonicalize().unwrap().to_str().unwrap())
                .unwrap();
        unsafe { gstreamer::agst_set_file(self.inner, cstring.as_ptr()) };
        self.play();
        self.pause();
    }

    pub fn play(&self) {
        unsafe { gstreamer::agst_play(self.inner) };
    }

    pub fn pause(&self) {
        unsafe { gstreamer::agst_pause(self.inner) };
    }

    pub fn is_playing(&self) -> bool {
        unsafe { gstreamer::agst_is_playing(self.inner) == 1 }
    }

    #[allow(dead_code)]
    pub fn get_duration(&self) -> i64 {
        unsafe { gstreamer::agst_duration(self.inner) }
    }

    pub fn get_position(&self) -> i64 {
        unsafe { gstreamer::agst_position(self.inner) }
    }

    pub fn set_position(&self, sec: i64) {
        unsafe { gstreamer::agst_seek(self.inner, sec) };
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        unsafe { gstreamer::agst_cleanup(self.inner) };
    }
}
