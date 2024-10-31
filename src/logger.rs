use std::ffi::CStr;
use std::os::raw::c_uint;
use crate::bindings::{hook_set_logger_proc, _log_level_LOG_LEVEL_DEBUG, _log_level_LOG_LEVEL_INFO, _log_level_LOG_LEVEL_WARN, _log_level_LOG_LEVEL_ERROR};

#[cfg(feature = "nightly")]
unsafe extern "C" fn logger(level: c_uint, message: *const i8, _: ...) -> bool {
    // Convert the C string message to a Rust string for output
    if let Ok(c_str) = CStr::from_ptr(message).to_str() {
        match level {
            _log_level_LOG_LEVEL_DEBUG => println!("[DEBUG]: {}", c_str),
            _log_level_LOG_LEVEL_INFO => println!("[INFO]: {}", c_str),
            _log_level_LOG_LEVEL_WARN => eprintln!("[WARN]: {}", c_str),
            _log_level_LOG_LEVEL_ERROR => eprintln!("[ERROR]: {}", c_str),
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
        hook_set_logger_proc(Some(logger));
    }
}
