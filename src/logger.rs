use std::{ffi::CStr, os::raw::c_uint};
use crate::bindings;

#[cfg(feature = "nightly")]
unsafe extern "C" fn logger(level: c_uint, message: *const i8, _: ...) -> bool {
    // Convert the C string message to a Rust string for output
    if let Ok(c_str) = CStr::from_ptr(message).to_str() {
        match level {
            bindings::_log_level_LOG_LEVEL_DEBUG => println!("[DEBUG]: {}", c_str),
            bindings::_log_level_LOG_LEVEL_INFO => println!("[INFO]: {}", c_str),
            bindings::_log_level_LOG_LEVEL_WARN => eprintln!("[WARN]: {}", c_str),
            bindings::_log_level_LOG_LEVEL_ERROR => eprintln!("[ERROR]: {}", c_str),
            _ => eprintln!("[UNKNOWN]: {}", c_str),
        }
        true
    } else {
        eprintln!("[ERROR]: Failed to read log message");
        false
    }
}

#[cfg(not(feature = "nightly"))]
unsafe extern "C" fn logger(_: c_uint, _: *const i8) -> bool {
    eprintln!("[ERROR]: Logger unavailable. Enable nightly features.");
    false
}

#[cfg(feature = "nightly")]
pub fn init_logger() {
    unsafe {
    bindings::hook_set_logger_proc(Some(logger));
    }
}
