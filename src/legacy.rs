//! Legacy module for backward compatibility with previous versions of uiohook-rs.
//!
//! This module provides the old API interface while internally using the new API structure.
//! Users of the previous version can continue to use these functions, but they are encouraged
//! to migrate to the new API for future development.

use crate::{
    Uiohook, EventHandler, UiohookEvent,
    keyboard::{KeyboardEvent as NewKeyboardEvent, KeyboardEventType as NewKeyboardEventType, KeyCode},
    mouse::{MouseEvent as NewMouseEvent, MouseEventType as NewMouseEventType, MouseButton},
    wheel::WheelEvent,
    error::UiohookError,
};
use std::sync::{Arc, Mutex, Once};
use lazy_static::lazy_static;
use std::collections::HashMap;

static INIT: Once = Once::new();
static mut GLOBAL_HANDLER: Option<Box<dyn Fn(&UiohookEvent)>> = None;

lazy_static! {
    static ref UIOHOOK: Mutex<Option<Uiohook>> = Mutex::new(None);
    static ref KEY_NAMES: HashMap<KeyCode, &'static str> = {
        let mut m = HashMap::new();
        m.insert(KeyCode::Escape, "Escape");
        m.insert(KeyCode::F1, "F1");
        // ... (add all other key mappings here)
        m
    };
}

/// Legacy keyboard event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardEventType {
    Pressed,
    Released,
    Typed,
}

/// Legacy keyboard event structure.
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub event_type: KeyboardEventType,
    pub key_code: u16,
    pub key_raw: u16,
    pub key_char: Option<char>,
    pub key_name: Option<&'static str>,
}

impl From<&NewKeyboardEvent> for KeyboardEvent {
    fn from(event: &NewKeyboardEvent) -> Self {
        KeyboardEvent {
            event_type: match event.event_type {
                NewKeyboardEventType::Pressed => KeyboardEventType::Pressed,
                NewKeyboardEventType::Released => KeyboardEventType::Released,
                NewKeyboardEventType::Typed => KeyboardEventType::Typed,
            },
            key_code: event.key_code as u16,
            key_raw: event.raw_code,
            key_char: event.key_char,
            key_name: KEY_NAMES.get(&event.key_code).copied(),
        }
    }
}

/// Legacy mouse event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Moved,
    Pressed,
    Released,
    Clicked,
    Dragged,
    Wheel,
}

/// Legacy mouse event structure.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub clicks: u16,
    pub x: i16,
    pub y: i16,
    pub button: u16,
    pub amount: u16,
    pub rotation: i16,
    pub direction: u8,
}

impl From<&NewMouseEvent> for MouseEvent {
    fn from(event: &NewMouseEvent) -> Self {
        MouseEvent {
            event_type: match event.event_type {
                NewMouseEventType::Moved => MouseEventType::Moved,
                NewMouseEventType::Pressed => MouseEventType::Pressed,
                NewMouseEventType::Released => MouseEventType::Released,
                NewMouseEventType::Clicked => MouseEventType::Clicked,
                NewMouseEventType::Dragged => MouseEventType::Dragged,
            },
            clicks: event.clicks,
            x: event.x,
            y: event.y,
            button: event.button as u16,
            amount: 0,
            rotation: 0,
            direction: 0,
        }
    }
}

impl From<&WheelEvent> for MouseEvent {
    fn from(event: &WheelEvent) -> Self {
        MouseEvent {
            event_type: MouseEventType::Wheel,
            clicks: event.clicks,
            x: event.x,
            y: event.y,
            button: 0,
            amount: event.amount,
            rotation: event.rotation,
            direction: event.direction,
        }
    }
}

/// Sets the dispatch procedure for handling uiohook events.
///
/// This function is provided for backward compatibility. It internally creates a new
/// Uiohook instance with a custom EventHandler that calls the provided callback.
///
/// # Arguments
///
/// * `callback` - A closure that takes a reference to a UiohookEvent and returns nothing.
///
/// # Example
///
/// ```no_run
/// use uiohook_rs::legacy::set_dispatch_proc;
/// use uiohook_rs::UiohookEvent;
///
/// set_dispatch_proc(|event: &UiohookEvent| {
///     println!("Event received: {:?}", event);
/// });
/// ```
pub fn set_dispatch_proc<F>(callback: F)
where
    F: Fn(&UiohookEvent) + 'static,
{
    INIT.call_once(|| {
        unsafe {
            GLOBAL_HANDLER = Some(Box::new(callback));
        }
    });

    struct LegacyHandler;

    impl EventHandler for LegacyHandler {
        fn handle_event(&self, event: &UiohookEvent) {
            if let Some(handler) = unsafe { GLOBAL_HANDLER.as_ref() } {
                handler(event);
            }
        }
    }

    let uiohook = Uiohook::new(LegacyHandler);
    *UIOHOOK.lock().unwrap() = Some(uiohook);
}

/// Runs the uiohook event loop.
///
/// This function is provided for backward compatibility. It internally calls the
/// `run` method on the Uiohook instance created by `set_dispatch_proc`.
///
/// # Returns
///
/// Returns `Ok(())` if the hook runs successfully, or an `Err(UiohookError)` if it fails.
///
/// # Example
///
/// ```no_run
/// use uiohook_rs::legacy::{set_dispatch_proc, run};
/// use uiohook_rs::UiohookEvent;
///
/// set_dispatch_proc(|event: &UiohookEvent| {
///     println!("Event received: {:?}", event);
/// });
///
/// if let Err(e) = run() {
///     eprintln!("Failed to run uiohook: {}", e);
/// }
/// ```
pub fn run() -> Result<(), UiohookError> {
    if let Some(uiohook) = UIOHOOK.lock().unwrap().as_ref() {
        uiohook.run()
    } else {
        Err(UiohookError::NotInitialized)
    }
}

/// Stops the uiohook event loop.
///
/// This function is provided for backward compatibility. It internally calls the
/// `stop` method on the Uiohook instance created by `set_dispatch_proc`.
///
/// # Returns
///
/// Returns `Ok(())` if the hook stops successfully, or an `Err(UiohookError)` if it fails.
///
/// # Example
///
/// ```no_run
/// use uiohook_rs::legacy::{set_dispatch_proc, run, stop};
/// use uiohook_rs::UiohookEvent;
/// use std::thread;
/// use std::time::Duration;
///
/// set_dispatch_proc(|event: &UiohookEvent| {
///     println!("Event received: {:?}", event);
/// });
///
/// let hook_thread = thread::spawn(|| {
///     if let Err(e) = run() {
///         eprintln!("Failed to run uiohook: {}", e);
///     }
/// });
///
/// thread::sleep(Duration::from_secs(5));
///
/// if let Err(e) = stop() {
///     eprintln!("Failed to stop uiohook: {}", e);
/// }
///
/// hook_thread.join().unwrap();
/// ```
pub fn stop() -> Result<(), UiohookError> {
    Uiohook::stop()
}

/// Sets the logger procedure for uiohook.
///
/// This function is provided for backward compatibility. It internally calls the
/// `set_logger_proc` function from the bindings module.
///
/// # Arguments
///
/// * `logger_proc` - A function pointer to the logger procedure.
///
/// # Safety
///
/// This function is unsafe because it directly interacts with C code.
pub unsafe fn set_logger_proc(logger_proc: crate::bindings::logger_t) {
    crate::bindings::hook_set_logger_proc(logger_proc);
}

/// Posts a uiohook event.
///
/// This function is provided for backward compatibility. It internally calls the
/// `post_event` method on the Uiohook instance created by `set_dispatch_proc`.
///
/// # Arguments
///
/// * `event` - A mutable reference to a raw uiohook_event.
///
/// # Safety
///
/// This function is unsafe because it directly interacts with C code.
pub unsafe fn post_event(event: &mut crate::bindings::uiohook_event) {
    if let Some(uiohook) = UIOHOOK.lock().unwrap().as_ref() {
        let _ = uiohook.post_event(&UiohookEvent::from(&*event));
    }
}

/// Handles a keyboard event and returns a legacy KeyboardEvent.
///
/// # Arguments
///
/// * `event` - A reference to a UiohookEvent.
///
/// # Returns
///
/// An Option containing a legacy KeyboardEvent if the event is a keyboard event, or None otherwise.
pub fn handle_keyboard_event(event: &UiohookEvent) -> Option<KeyboardEvent> {
    match event {
        UiohookEvent::Keyboard(ke) => Some(KeyboardEvent::from(ke)),
        _ => None,
    }
}

/// Handles a mouse event and returns a legacy MouseEvent.
///
/// # Arguments
///
/// * `event` - A reference to a UiohookEvent.
///
/// # Returns
///
/// An Option containing a legacy MouseEvent if the event is a mouse or wheel event, or None otherwise.
pub fn handle_mouse_event(event: &UiohookEvent) -> Option<MouseEvent> {
    match event {
        UiohookEvent::Mouse(me) => Some(MouseEvent::from(me)),
        UiohookEvent::Wheel(we) => Some(MouseEvent::from(we)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::KeyCode;
    use crate::mouse::MouseButton;

    #[test]
    fn test_legacy_keyboard_event_conversion() {
        let new_event = NewKeyboardEvent {
            event_type: NewKeyboardEventType::Pressed,
            key_code: KeyCode::A,
            raw_code: 65,
            key_char: Some('A'),
        };

        let legacy_event = KeyboardEvent::from(&new_event);

        assert_eq!(legacy_event.event_type, KeyboardEventType::Pressed);
        assert_eq!(legacy_event.key_code, KeyCode::A as u16);
        assert_eq!(legacy_event.key_raw, 65);
        assert_eq!(legacy_event.key_char, Some('A'));
        assert_eq!(legacy_event.key_name, Some("A"));
    }

    #[test]
    fn test_legacy_mouse_event_conversion() {
        let new_event = NewMouseEvent {
            event_type: NewMouseEventType::Clicked,
            button: MouseButton::Button1,
            clicks: 1,
            x: 100,
            y: 200,
        };

        let legacy_event = MouseEvent::from(&new_event);

        assert_eq!(legacy_event.event_type, MouseEventType::Clicked);
        assert_eq!(legacy_event.button, MouseButton::Button1 as u16);
        assert_eq!(legacy_event.clicks, 1);
        assert_eq!(legacy_event.x, 100);
        assert_eq!(legacy_event.y, 200);
    }

    #[test]
    fn test_legacy_wheel_event_conversion() {
        let wheel_event = WheelEvent {
            clicks: 1,
            x: 100,
            y: 200,
            type_: 1,
            amount: 120,
            rotation: 1,
            direction: 3,
        };

        let legacy_event = MouseEvent::from(&wheel_event);

        assert_eq!(legacy_event.event_type, MouseEventType::Wheel);
        assert_eq!(legacy_event.clicks, 1);
        assert_eq!(legacy_event.x, 100);
        assert_eq!(legacy_event.y, 200);
        assert_eq!(legacy_event.amount, 120);
        assert_eq!(legacy_event.rotation, 1);
        assert_eq!(legacy_event.direction, 3);
    }
}