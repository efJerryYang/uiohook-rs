//! Core functionality for hooking keyboard and mouse events.
//!
//! This module provides the main `Uiohook` struct and the `EventHandler` trait
//! for handling uiohook events.

use crate::{bindings, KeyboardEventType, MouseEventType};
use crate::error::UiohookError;
use self::keyboard::KeyboardEvent;
use self::mouse::MouseEvent;
use self::wheel::WheelEvent;
// use std::ptr::addr_of_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock, Once, OnceLock};
use std::thread;

pub mod keyboard;
pub mod mouse;
pub mod wheel;

static INIT: Once = Once::new();
static GLOBAL_HANDLER: OnceLock<Arc<RwLock<dyn EventHandler>>> = OnceLock::new();

/// Trait for handling uiohook events.
pub trait EventHandler: Send + Sync {
    /// Handle a uiohook event.
    fn handle_event(&self, event: &UiohookEvent);
}

/// Main struct for interacting with uiohook.
pub struct Uiohook {
    event_handler: Arc<RwLock<dyn EventHandler>>,
    running: Arc<AtomicBool>,
    thread_handle: RwLock<Option<thread::JoinHandle<()>>>,
}


impl Uiohook {
    /// Create a new Uiohook instance with the given event handler.
    ///
    /// # Arguments
    ///
    /// * `event_handler` - An implementation of the `EventHandler` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent};
    ///
    /// struct MyHandler;
    ///
    /// impl EventHandler for MyHandler {
    ///     fn handle_event(&self, event: &UiohookEvent) {
    ///         println!("Event: {:?}", event);
    ///     }
    /// }
    ///
    /// let hook = Uiohook::new(MyHandler);
    /// ```
    pub fn new<H: EventHandler + 'static>(event_handler: H) -> Self {
        Self {
            event_handler: Arc::new(RwLock::new(event_handler)),
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: RwLock::new(None),
        }
    }

     /// Run the uiohook event loop.
    ///
    /// This method will block until `stop()` is called or an error occurs.
    ///
    /// # Errors
    ///
    /// Returns a `UiohookError` if the hook fails to start or encounters an error while running.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent};
    ///
    /// struct MyHandler;
    ///
    /// impl EventHandler for MyHandler {
    ///     fn handle_event(&self, event: &UiohookEvent) {
    ///         println!("Event: {:?}", event);
    ///     }
    /// }
    ///
    /// let hook = Uiohook::new(MyHandler);
    /// hook.run().expect("Failed to run uiohook");
    /// ```
    pub fn run(&self) -> Result<(), UiohookError> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err(UiohookError::AlreadyRunning);
        }

        INIT.call_once(|| {
            unsafe {
                if GLOBAL_HANDLER.set(Arc::clone(&self.event_handler)).is_err() {
                    eprintln!("Failed to set global handler");
                }
                bindings::hook_set_dispatch_proc(Some(dispatch_proc_wrapper));
            }
        });

        let running = self.running.clone();
        let thread = thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let result = unsafe { bindings::hook_run() };
                if result != bindings::UIOHOOK_SUCCESS as i32 {
                    eprintln!("Error in hook_run: {:?}", UiohookError::from(result as u32));
                    break;
                }
            }
        });

        *self.thread_handle.write().unwrap() = Some(thread);

        Ok(())
    }

    /// Stop the uiohook event loop.
    ///
    /// # Errors
    ///
    /// Returns a `UiohookError` if the hook fails to stop.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent};
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// struct MyHandler;
    ///
    /// impl EventHandler for MyHandler {
    ///     fn handle_event(&self, event: &UiohookEvent) {
    ///         println!("Event: {:?}", event);
    ///     }
    /// }
    ///
    /// let hook = Uiohook::new(MyHandler);
    ///
    /// // Start the hook
    /// hook.run().expect("Failed to run uiohook");
    ///
    /// // Do something here...
    ///
    /// // Stop the hook
    /// hook.stop().expect("Failed to stop uiohook");
    /// ```
    pub fn stop(&self) -> Result<(), UiohookError> {
        if !self.running.swap(false, Ordering::SeqCst) {
            return Err(UiohookError::NotRunning);
        }

        let result = unsafe { bindings::hook_stop() };

        if let Some(thread) = self.thread_handle.write().unwrap().take() {
            thread.join().map_err(|_| UiohookError::Failure)?;
        }

        if result == bindings::UIOHOOK_SUCCESS as i32 {
            Ok(())
        } else {
            Err(UiohookError::from(result as u32))
        }
    }

    
    /// Post a synthetic event.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to post.
    ///
    /// # Errors
    ///
    /// Returns a `UiohookError` if the event fails to post.
    ///
    /// # Examples
    ///
    /// ```
    /// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent};
    /// use uiohook_rs::hook::keyboard::{KeyboardEvent, KeyboardEventType, KeyCode};
    ///
    /// struct MyHandler;
    ///
    /// impl EventHandler for MyHandler {
    ///     fn handle_event(&self, event: &UiohookEvent) {
    ///         println!("Event: {:?}", event);
    ///     }
    /// }
    ///
    /// let hook = Uiohook::new(MyHandler);
    ///
    /// // Create a keyboard event
    /// let key_event = KeyboardEvent {
    ///     event_type: KeyboardEventType::Pressed,
    ///     key_code: KeyCode::A,
    ///     raw_code: 0x41,
    ///     key_char: Some('A'),
    /// };
    ///
    /// // In a real scenario, you would run the hook before posting events
    /// hook.run().expect("Failed to run uiohook");
    ///
    /// // Demonstrate how to use post_event (this won't actually post the event in the doc test)
    /// // hook.post_event(&UiohookEvent::Keyboard(key_event)).expect("Failed to post event");
    ///
    /// // For the purpose of this example, we'll just print the event
    /// println!("Would post event: {:?}", UiohookEvent::Keyboard(key_event));
    /// ```
    pub fn post_event(&self, event: &UiohookEvent) -> Result<(), UiohookError> {
        let mut raw_event = event.to_raw_event();
        unsafe {
            bindings::hook_post_event(&mut raw_event);
        }
        Ok(())
    }
}

/// Enumeration of possible uiohook events.
#[derive(Debug, Clone)]
pub enum UiohookEvent {
    /// Keyboard event (key press, release, or type)
    Keyboard(KeyboardEvent),
    /// Mouse event (movement, button press, release, or click)
    Mouse(MouseEvent),
    /// Mouse wheel event
    Wheel(WheelEvent),
    /// Hook enabled event
    HookEnabled,
    /// Hook disabled event
    HookDisabled,
}


impl UiohookEvent {
    fn from_raw_event(event: &bindings::uiohook_event) -> Self {
        use bindings::event_type::*;
        match event.type_ {
            EVENT_HOOK_ENABLED => UiohookEvent::HookEnabled,
            EVENT_HOOK_DISABLED => UiohookEvent::HookDisabled,
            EVENT_KEY_PRESSED | EVENT_KEY_RELEASED | EVENT_KEY_TYPED => {
                UiohookEvent::Keyboard(Self::create_keyboard_event(event))
            }
            EVENT_MOUSE_CLICKED | EVENT_MOUSE_PRESSED | EVENT_MOUSE_RELEASED
            | EVENT_MOUSE_MOVED | EVENT_MOUSE_DRAGGED => {
                UiohookEvent::Mouse(Self::create_mouse_event(event))
            }
            EVENT_MOUSE_WHEEL => {
                UiohookEvent::Wheel(Self::create_wheel_event(event))
            }
        }
    }

    fn create_keyboard_event(event: &bindings::uiohook_event) -> KeyboardEvent {
        use bindings::event_type::*;
        let mut ke = KeyboardEvent::from(unsafe { &event.data.keyboard });
        ke.event_type = match event.type_ {
            EVENT_KEY_PRESSED => KeyboardEventType::Pressed,
            EVENT_KEY_RELEASED => KeyboardEventType::Released,
            EVENT_KEY_TYPED => KeyboardEventType::Typed,
            _ => unreachable!(),
        };
        ke
    }

    fn create_mouse_event(event: &bindings::uiohook_event) -> MouseEvent {
        use bindings::event_type::*;
        let mut me = MouseEvent::from(unsafe { &event.data.mouse });
        me.event_type = match event.type_ {
            EVENT_MOUSE_CLICKED => MouseEventType::Clicked,
            EVENT_MOUSE_PRESSED => MouseEventType::Pressed,
            EVENT_MOUSE_RELEASED => MouseEventType::Released,
            EVENT_MOUSE_MOVED => MouseEventType::Moved,
            EVENT_MOUSE_DRAGGED => MouseEventType::Dragged,
            _ => unreachable!(),
        };
        me
    }

    fn create_wheel_event(event: &bindings::uiohook_event) -> WheelEvent {
        WheelEvent::from(unsafe { &event.data.wheel })
    }


    fn to_raw_event(&self) -> bindings::uiohook_event {
        use bindings::event_type::*;
        let mut raw_event: bindings::uiohook_event = unsafe { std::mem::zeroed() };

        match self {
            UiohookEvent::HookEnabled => {
                raw_event.type_ = EVENT_HOOK_ENABLED;
            }
            UiohookEvent::HookDisabled => {
                raw_event.type_ = EVENT_HOOK_DISABLED;
            }
            UiohookEvent::Keyboard(ke) => {
                raw_event.type_ = match ke.event_type {
                    KeyboardEventType::Pressed => EVENT_KEY_PRESSED,
                    KeyboardEventType::Released => EVENT_KEY_RELEASED,
                    KeyboardEventType::Typed => EVENT_KEY_TYPED,
                };
                raw_event.data.keyboard.keycode = ke.key_code as u16;
                raw_event.data.keyboard.rawcode = ke.raw_code;
                raw_event.data.keyboard.keychar = ke.key_char.map(|c| c as u16).unwrap_or(0);
            }
            UiohookEvent::Mouse(me) => {
                raw_event.type_ = match me.event_type {
                    MouseEventType::Moved => EVENT_MOUSE_MOVED,
                    MouseEventType::Pressed => EVENT_MOUSE_PRESSED,
                    MouseEventType::Released => EVENT_MOUSE_RELEASED,
                    MouseEventType::Clicked => EVENT_MOUSE_CLICKED,
                    MouseEventType::Dragged => EVENT_MOUSE_DRAGGED,
                };
                raw_event.data.mouse.button = me.button as u16;
                raw_event.data.mouse.clicks = me.clicks;
                raw_event.data.mouse.x = me.x;
                raw_event.data.mouse.y = me.y;
            }
            UiohookEvent::Wheel(we) => {
                raw_event.type_ = EVENT_MOUSE_WHEEL;
                raw_event.data.wheel.clicks = we.clicks;
                raw_event.data.wheel.x = we.x;
                raw_event.data.wheel.y = we.y;
                raw_event.data.wheel.type_ = we.type_;
                raw_event.data.wheel.amount = we.amount;
                raw_event.data.wheel.rotation = we.rotation;
                raw_event.data.wheel.direction = we.direction;
            }
        }

        raw_event
    }
}

impl From<&bindings::uiohook_event> for UiohookEvent {
    fn from(event: &bindings::uiohook_event) -> Self {
        UiohookEvent::from_raw_event(&event)
    }
}

unsafe extern "C" fn dispatch_proc_wrapper(event: *mut bindings::uiohook_event) {
    dispatch_proc(&*event);
}

fn dispatch_proc(event: &bindings::uiohook_event) {
    if let Some(handler) = GLOBAL_HANDLER.get() {
        let event = UiohookEvent::from_raw_event(event);
        if let Ok(guard) = handler.read() {
            guard.handle_event(&event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::Duration;

    struct TestHandler {
        event_count: Arc<AtomicUsize>,
    }

    impl EventHandler for TestHandler {
        fn handle_event(&self, _event: &UiohookEvent) {
            self.event_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_uiohook_run_and_stop() {
        let running = Arc::new(AtomicBool::new(true));
        let event_count = Arc::new(AtomicUsize::new(0));
        let handler = TestHandler { 
            event_count: event_count.clone(),
        };
        
        let hook = Uiohook::new(handler);

        // Run the hook
        if let Err(e) = hook.run() {
            panic!("Failed to run uiohook: {}", e);
        }

        // Wait for the hook to start
        std::thread::sleep(Duration::from_millis(100));

        // Post a test event
        let test_event = UiohookEvent::Keyboard(KeyboardEvent {
            event_type: self::keyboard::KeyboardEventType::Pressed,
            key_code: self::keyboard::KeyCode::A,
            raw_code: 0x41,
            key_char: Some('A'),
        });
        hook.post_event(&test_event).expect("Failed to post event");

        // Wait for the event to be processed
        std::thread::sleep(Duration::from_millis(100));

        // Check if the event was processed
        assert_eq!(event_count.load(Ordering::SeqCst), 1, "Event was not processed");

        // Stop the hook
        running.store(false, Ordering::SeqCst);
        hook.stop().expect("Failed to stop uiohook");

        // Ensure the hook has stopped
        std::thread::sleep(Duration::from_millis(100));
    }
}