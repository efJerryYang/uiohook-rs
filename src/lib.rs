// Import the generated bindings
mod bindings;
use bindings::*;

// Re-export all constants
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
};

// Re-export event types and data structures
pub use bindings::{
    event_type,
    screen_data,
    keyboard_event_data,
    mouse_event_data,
    mouse_wheel_event_data,
    uiohook_event,
};

// Re-export function types
pub use bindings::{
    logger_t,
    dispatcher_t,
};

// Re-export all virtual key codes
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
    VC_KP_DIVIDE, VC_KP_MULTIPLY, VC_KP_SUBTRACT, VC_KP_EQUALS, VC_KP_ADD, VC_KP_ENTER, VC_KP_SEPARATOR,
    VC_KP_1, VC_KP_2, VC_KP_3, VC_KP_4, VC_KP_5, VC_KP_6, VC_KP_7, VC_KP_8, VC_KP_9, VC_KP_0,
    VC_KP_END, VC_KP_DOWN, VC_KP_PAGE_DOWN, VC_KP_LEFT, VC_KP_CLEAR, VC_KP_RIGHT,
    VC_KP_HOME, VC_KP_UP, VC_KP_PAGE_UP, VC_KP_INSERT, VC_KP_DELETE,
    VC_SHIFT_L, VC_SHIFT_R, VC_CONTROL_L, VC_CONTROL_R, VC_ALT_L, VC_ALT_R, VC_META_L, VC_META_R,
    VC_CONTEXT_MENU,
    VC_POWER, VC_SLEEP, VC_WAKE,
    VC_MEDIA_PLAY, VC_MEDIA_STOP, VC_MEDIA_PREVIOUS, VC_MEDIA_NEXT, VC_MEDIA_SELECT, VC_MEDIA_EJECT,
    VC_VOLUME_MUTE, VC_VOLUME_UP, VC_VOLUME_DOWN,
    VC_APP_MAIL, VC_APP_CALCULATOR, VC_APP_MUSIC, VC_APP_PICTURES,
    VC_BROWSER_SEARCH, VC_BROWSER_HOME, VC_BROWSER_BACK, VC_BROWSER_FORWARD,
    VC_BROWSER_STOP, VC_BROWSER_REFRESH, VC_BROWSER_FAVORITES,
    VC_KATAKANA, VC_UNDERSCORE, VC_FURIGANA, VC_KANJI, VC_HIRAGANA, VC_YEN, VC_KP_COMMA,
    VC_SUN_HELP, VC_SUN_STOP, VC_SUN_PROPS, VC_SUN_FRONT, VC_SUN_OPEN, VC_SUN_FIND,
    VC_SUN_AGAIN, VC_SUN_UNDO, VC_SUN_COPY, VC_SUN_INSERT, VC_SUN_CUT,
    VC_UNDEFINED,
    CHAR_UNDEFINED,
};

// Re-export all modifier masks
pub use bindings::{
    MASK_SHIFT_L, MASK_CTRL_L, MASK_META_L, MASK_ALT_L,
    MASK_SHIFT_R, MASK_CTRL_R, MASK_META_R, MASK_ALT_R,
    MASK_SHIFT, MASK_CTRL, MASK_META, MASK_ALT,
    MASK_BUTTON1, MASK_BUTTON2, MASK_BUTTON3, MASK_BUTTON4, MASK_BUTTON5,
    MASK_NUM_LOCK, MASK_CAPS_LOCK, MASK_SCROLL_LOCK,
};

// Re-export mouse button constants
pub use bindings::{
    MOUSE_NOBUTTON, MOUSE_BUTTON1, MOUSE_BUTTON2, MOUSE_BUTTON3, MOUSE_BUTTON4, MOUSE_BUTTON5,
    WHEEL_UNIT_SCROLL, WHEEL_BLOCK_SCROLL,
    WHEEL_VERTICAL_DIRECTION, WHEEL_HORIZONTAL_DIRECTION,
};

// Wrapper functions for the C API

/// Set the logger callback function.
pub fn set_logger_proc(logger_proc: logger_t) {
    unsafe { hook_set_logger_proc(logger_proc) }
}

/// Send a virtual event back to the system.
pub fn post_event(event: &mut uiohook_event) {
    unsafe { hook_post_event(event) }
}

/// Set the event callback function.
pub fn set_dispatch_proc(dispatch_proc: dispatcher_t) {
    unsafe { hook_set_dispatch_proc(dispatch_proc) }
}

/// Insert the event hook.
pub fn run() -> i32 {
    unsafe { hook_run() }
}

/// Withdraw the event hook.
pub fn stop() -> i32 {
    unsafe { hook_stop() }
}

/// Retrieves an array of screen data for each available monitor.
pub fn create_screen_info() -> Vec<screen_data> {
    let mut count: u8 = 0;
    let ptr = unsafe { hook_create_screen_info(&mut count) };
    let slice = unsafe { std::slice::from_raw_parts(ptr, count as usize) };
    let vec = slice.to_vec();
    unsafe { libc::free(ptr as *mut libc::c_void) };
    vec
}

/// Retrieves the keyboard auto repeat rate.
pub fn get_auto_repeat_rate() -> i64 {
    unsafe { hook_get_auto_repeat_rate() }
}

/// Retrieves the keyboard auto repeat delay.
pub fn get_auto_repeat_delay() -> i64 {
    unsafe { hook_get_auto_repeat_delay() }
}

/// Retrieves the mouse acceleration multiplier.
pub fn get_pointer_acceleration_multiplier() -> i64 {
    unsafe { hook_get_pointer_acceleration_multiplier() }
}

/// Retrieves the mouse acceleration threshold.
pub fn get_pointer_acceleration_threshold() -> i64 {
    unsafe { hook_get_pointer_acceleration_threshold() }
}

/// Retrieves the mouse sensitivity.
pub fn get_pointer_sensitivity() -> i64 {
    unsafe { hook_get_pointer_sensitivity() }
}

/// Retrieves the double/triple click interval.
pub fn get_multi_click_time() -> i64 {
    unsafe { hook_get_multi_click_time() }
}

// Utility functions for working with events

/// Get a reference to the keyboard event data from a uiohook_event.
pub fn get_keyboard_event(event: &uiohook_event) -> &keyboard_event_data {
    unsafe { &event.data.keyboard }
}

/// Get a reference to the mouse event data from a uiohook_event.
pub fn get_mouse_event(event: &uiohook_event) -> &mouse_event_data {
    unsafe { &event.data.mouse }
}

/// Get a reference to the wheel event data from a uiohook_event.
pub fn get_wheel_event(event: &uiohook_event) -> &mouse_wheel_event_data {
    unsafe { &event.data.wheel }
}

// TODO: Any additional utility functions or safe wrappers here