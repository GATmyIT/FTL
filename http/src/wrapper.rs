use libc;
use std::ffi::CString;

extern {
    fn logg(format: *const libc::c_char, ...);
}

pub fn log(msg: &str) {
    unsafe {
        let c_str = CString::new(msg).unwrap();
        logg(c_str.as_ptr());
    }
}


