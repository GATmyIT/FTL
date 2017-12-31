use libc;
use std::ffi::CString;

extern {
    //noinspection RsStaticConstNaming
    static debug: bool;

    fn logg(format: *const libc::c_char, ...);
}

pub fn is_debug() -> bool {
    unsafe { debug }
}

pub fn log(msg: &str) {
    unsafe {
        let c_str = CString::new(msg).unwrap();
        logg(c_str.as_ptr());
    }
}
