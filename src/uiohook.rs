use crate::bindings;
use crate::error::UiohookError;
use crate::keyboard::KeyboardEvent;
use crate::mouse::MouseEvent;
use crate::wheel::WheelEvent;
use std::ptr::addr_of_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Once};

static INIT: Once = Once::new();
static mut GLOBAL_HANDLER: Option<Arc<Mutex<dyn EventHandler>>> = None;
static RUNNING: AtomicBool = AtomicBool::new(false);

/// Trait for handling uiohook events.
pub trait EventHandler: Send + Sync {
    /// Handle a uiohook event.
    fn handle_event(&self, event: &UiohookEvent);
}

/// Main struct for interacting with uiohook.
pub struct Uiohook {
    event_handler: Arc<Mutex<dyn EventHandler>>,
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
    /// ```
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
            event_handler: Arc::new(Mutex::new(event_handler)),
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
        if RUNNING.swap(true, Ordering::SeqCst) {
            return Err(UiohookError::AlreadyRunning);
        }

        INIT.call_once(|| {
            unsafe {
                GLOBAL_HANDLER = Some(Arc::clone(&self.event_handler));
                bindings::hook_set_dispatch_proc(Some(dispatch_proc_wrapper));
            }
        });

        let result = unsafe { bindings::hook_run() };

        if result == bindings::UIOHOOK_SUCCESS as i32 {
            Ok(())
        } else {
            RUNNING.store(false, Ordering::SeqCst);
            Err(UiohookError::from(result as u32))
        }
    }

    /// Stop the uiohook event loop.
    ///
    /// # Errors
    ///
    /// Returns a `UiohookError` if the hook fails to stop.
    ///
    /// # Examples
    ///
    /// ```no_run
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
    /// let hook_thread = thread::spawn(move || {
    ///     hook.run().expect("Failed to run uiohook");
    /// });
    ///
    /// thread::sleep(Duration::from_secs(5));
    ///
    /// Uiohook::stop().expect("Failed to stop uiohook");
    /// hook_thread.join().expect("Failed to join hook thread");
    /// ```
    pub fn stop() -> Result<(), UiohookError> {
        if !RUNNING.swap(false, Ordering::SeqCst) {
            return Err(UiohookError::NotRunning);
        }

        let result = unsafe { bindings::hook_stop() };

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
    /// ```no_run
    /// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, KeyboardEvent, KeyboardEventType};
    /// use uiohook_rs::keyboard::KeyCode;
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
    /// let key_event = KeyboardEvent {
    ///     event_type: KeyboardEventType::Pressed,
    ///     key_code: KeyCode::A,
    ///     raw_code: 0x41,
    ///     key_char: Some('A'),
    /// };
    ///
    /// hook.post_event(&UiohookEvent::Keyboard(key_event)).expect("Failed to post event");
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
                UiohookEvent::Keyboard(KeyboardEvent::from(unsafe { &event.data.keyboard }))
            }
            EVENT_MOUSE_CLICKED | EVENT_MOUSE_PRESSED | EVENT_MOUSE_RELEASED
            | EVENT_MOUSE_MOVED | EVENT_MOUSE_DRAGGED => {
                UiohookEvent::Mouse(MouseEvent::from(unsafe { &event.data.mouse }))
            }
            EVENT_MOUSE_WHEEL => {
                UiohookEvent::Wheel(WheelEvent::from(unsafe { &event.data.wheel }))
            }
            _ => panic!("Unknown event type: {:?}", event.type_),
        }
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
                    crate::keyboard::KeyboardEventType::Pressed => EVENT_KEY_PRESSED,
                    crate::keyboard::KeyboardEventType::Released => EVENT_KEY_RELEASED,
                    crate::keyboard::KeyboardEventType::Typed => EVENT_KEY_TYPED,
                };
                raw_event.data.keyboard.keycode = ke.key_code as u16;
                raw_event.data.keyboard.rawcode = ke.raw_code;
                raw_event.data.keyboard.keychar = ke.key_char.map(|c| c as u16).unwrap_or(0);
            }
            UiohookEvent::Mouse(me) => {
                raw_event.type_ = match me.event_type {
                    crate::mouse::MouseEventType::Moved => EVENT_MOUSE_MOVED,
                    crate::mouse::MouseEventType::Pressed => EVENT_MOUSE_PRESSED,
                    crate::mouse::MouseEventType::Released => EVENT_MOUSE_RELEASED,
                    crate::mouse::MouseEventType::Clicked => EVENT_MOUSE_CLICKED,
                    crate::mouse::MouseEventType::Dragged => EVENT_MOUSE_DRAGGED,
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
    if let Some(handler) = unsafe { GLOBAL_HANDLER.as_ref() } {
        let event = UiohookEvent::from_raw_event(event);
        if let Ok(guard) = handler.lock() {
            guard.handle_event(&event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    struct TestHandler {
        sender: mpsc::Sender<UiohookEvent>,
    }

    impl EventHandler for TestHandler {
        fn handle_event(&self, event: &UiohookEvent) {
            self.sender.send(event.clone()).unwrap();
        }
    }

    #[test]
    fn test_uiohook_run_and_stop() {
        let (tx, rx) = mpsc::channel();
        let handler = TestHandler { sender: tx };
        let hook = Arc::new(Uiohook::new(handler));
        let hook_clone = Arc::clone(&hook);
        let hook_thread = std::thread::spawn(move || {
            hook_clone.run().expect("Failed to run uiohook");
        });

        // Wait for the hook to start
        std::thread::sleep(Duration::from_secs(1));

        // Post a test event
        let test_event = UiohookEvent::Keyboard(KeyboardEvent {
            event_type: crate::keyboard::KeyboardEventType::Pressed,
            key_code: crate::keyboard::KeyCode::A,
            raw_code: 0x41,
            key_char: Some('A'),
        });

        hook.post_event(&test_event).expect("Failed to post event");

        // Wait for the event to be processed
        let received_event = rx
            .recv_timeout(Duration::from_secs(5))
            .expect("Failed to receive event");

        assert!(matches!(received_event, UiohookEvent::Keyboard(_)));

        // Stop the hook
        Uiohook::stop().expect("Failed to stop uiohook");

        // Wait for the hook thread to finish
        hook_thread.join().expect("Failed to join hook thread");
    }
}
