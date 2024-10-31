//! # Example
//!
//! ```rust,no_run
//! use std::sync::atomic::{AtomicI32, Ordering};
//! use std::sync::Arc;
//! use std::thread;
//! use std::time::Duration;
//! use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};
//!
//! struct MyEventHandler {
//!     event_count: Arc<AtomicI32>,
//! }
//!
//! impl EventHandler for MyEventHandler {
//!     fn handle_event(&self, event: &UiohookEvent) {
//!         println!("Event: {:?}", event);
//!         self.event_count.fetch_add(1, Ordering::SeqCst);
//!     }
//! }
//!
//! fn main() {
//!     let event_count = Arc::new(AtomicI32::new(0));
//!     let event_handler = MyEventHandler {
//!         event_count: event_count.clone(),
//!     };
//!     let uiohook = Uiohook::new(event_handler);
//!
//!     uiohook.run().expect("Failed to run uiohook");
//!
//!     // Let the hook run for 5 seconds (in a real application, run a platform-specific event loop)
//!     thread::sleep(Duration::from_secs(5));
//!
//!     uiohook.stop().expect("Failed to stop uiohook");
//! }
//! ```

#![feature(c_variadic)]
#![allow(missing_docs)]

mod bindings;
pub mod hook;
pub mod error;
pub mod utils;
pub mod logger;
// pub mod legacy;

// Re-export the main components
pub use hook::{Uiohook, EventHandler, UiohookEvent};
pub use hook::keyboard::{KeyboardEvent, KeyboardEventType, key_tap, key_toggle};
pub use hook::mouse::{MouseEvent, MouseEventType};
pub use hook::wheel::WheelEvent;
pub use error::UiohookError;

// Re-export utility functions
pub use utils::{
    create_screen_info,
    get_auto_repeat_rate,
    get_auto_repeat_delay,
    get_pointer_acceleration_multiplier,
    get_pointer_acceleration_threshold,
    get_pointer_sensitivity,
    get_multi_click_time,
};

// Re-export constants from bindings
pub use bindings::{
    UIOHOOK_SUCCESS,
    UIOHOOK_FAILURE,
    UIOHOOK_ERROR_OUT_OF_MEMORY,
    UIOHOOK_ERROR_X_OPEN_DISPLAY,
    UIOHOOK_ERROR_X_RECORD_NOT_FOUND,
    UIOHOOK_ERROR_X_RECORD_ALLOC_RANGE,
    UIOHOOK_ERROR_X_RECORD_CREATE_CONTEXT,
    UIOHOOK_ERROR_X_RECORD_ENABLE_CONTEXT,
    UIOHOOK_ERROR_X_RECORD_GET_CONTEXT,
    UIOHOOK_ERROR_SET_WINDOWS_HOOK_EX,
    UIOHOOK_ERROR_GET_MODULE_HANDLE,
    UIOHOOK_ERROR_AXAPI_DISABLED,
    UIOHOOK_ERROR_CREATE_EVENT_PORT,
    UIOHOOK_ERROR_CREATE_RUN_LOOP_SOURCE,
    UIOHOOK_ERROR_GET_RUNLOOP,
    UIOHOOK_ERROR_CREATE_OBSERVER,
    _log_level_LOG_LEVEL_DEBUG as LOG_LEVEL_DEBUG,
    _log_level_LOG_LEVEL_INFO as LOG_LEVEL_INFO,
    _log_level_LOG_LEVEL_WARN as LOG_LEVEL_WARN,
    _log_level_LOG_LEVEL_ERROR as LOG_LEVEL_ERROR,
};

// Re-export key codes
pub use bindings::{
    VC_ESCAPE,
    VC_F1, VC_F2, VC_F3, VC_F4, VC_F5, VC_F6, VC_F7, VC_F8, VC_F9, VC_F10, VC_F11, VC_F12,
    VC_F13, VC_F14, VC_F15, VC_F16, VC_F17, VC_F18, VC_F19, VC_F20, VC_F21, VC_F22, VC_F23, VC_F24,
    VC_BACKQUOTE,
    VC_1, VC_2, VC_3, VC_4, VC_5, VC_6, VC_7, VC_8, VC_9, VC_0,
    VC_MINUS, VC_EQUALS, VC_BACKSPACE,
    VC_TAB, VC_CAPS_LOCK,
    VC_A, VC_B, VC_C, VC_D, VC_E, VC_F, VC_G, VC_H, VC_I, VC_J, VC_K, VC_L, VC_M,
    VC_N, VC_O, VC_P, VC_Q, VC_R, VC_S, VC_T, VC_U, VC_V, VC_W, VC_X, VC_Y, VC_Z,
    VC_OPEN_BRACKET, VC_CLOSE_BRACKET, VC_BACK_SLASH,
    VC_SEMICOLON, VC_QUOTE, VC_ENTER,
    VC_COMMA, VC_PERIOD, VC_SLASH,
    VC_SPACE,
    VC_PRINTSCREEN, VC_SCROLL_LOCK, VC_PAUSE,
    VC_LESSER_GREATER,
    VC_INSERT, VC_DELETE, VC_HOME, VC_END, VC_PAGE_UP, VC_PAGE_DOWN,
    VC_UP, VC_LEFT, VC_CLEAR, VC_RIGHT, VC_DOWN,
    VC_NUM_LOCK,
    VC_KP_DIVIDE, VC_KP_MULTIPLY, VC_KP_SUBTRACT, VC_KP_EQUALS, VC_KP_ADD, VC_KP_ENTER,
    VC_KP_SEPARATOR,
    VC_KP_1, VC_KP_2, VC_KP_3, VC_KP_4, VC_KP_5, VC_KP_6, VC_KP_7, VC_KP_8, VC_KP_9, VC_KP_0,
    VC_SHIFT_L, VC_SHIFT_R, VC_CONTROL_L, VC_CONTROL_R, VC_ALT_L, VC_ALT_R, VC_META_L, VC_META_R,
    VC_CONTEXT_MENU,
};

// Re-export mouse button constants
pub use bindings::{
    MOUSE_BUTTON1, MOUSE_BUTTON2, MOUSE_BUTTON3, MOUSE_BUTTON4, MOUSE_BUTTON5,
    WHEEL_UNIT_SCROLL, WHEEL_BLOCK_SCROLL,
};

// Re-export modifier masks
pub use bindings::{
    MASK_SHIFT_L, MASK_CTRL_L, MASK_META_L, MASK_ALT_L,
    MASK_SHIFT_R, MASK_CTRL_R, MASK_META_R, MASK_ALT_R,
    MASK_SHIFT, MASK_CTRL, MASK_META, MASK_ALT,
};

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}