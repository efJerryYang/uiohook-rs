use crate::bindings;
use crate::error::UiohookError;
use crate::Uiohook;
use std::convert::TryFrom;

/// Represents the type of mouse event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    /// The mouse was moved.
    Moved,
    /// A mouse button was pressed.
    Pressed,
    /// A mouse button was released.
    Released,
    /// A mouse button was clicked (pressed and released).
    Clicked,
    /// The mouse was dragged (moved with a button pressed).
    Dragged,
}

/// Represents a mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// No button or an unknown button.
    NoButton,
    /// Left mouse button.
    Button1,
    /// Right mouse button.
    Button2,
    /// Middle mouse button.
    Button3,
    /// Additional mouse button 1.
    Button4,
    /// Additional mouse button 2.
    Button5,
}

impl TryFrom<u32> for MouseButton {
    type Error = UiohookError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            bindings::MOUSE_NOBUTTON => Ok(MouseButton::NoButton),
            bindings::MOUSE_BUTTON1 => Ok(MouseButton::Button1),
            bindings::MOUSE_BUTTON2 => Ok(MouseButton::Button2),
            bindings::MOUSE_BUTTON3 => Ok(MouseButton::Button3),
            bindings::MOUSE_BUTTON4 => Ok(MouseButton::Button4),
            bindings::MOUSE_BUTTON5 => Ok(MouseButton::Button5),
            _ => Err(UiohookError::UnknownMouseButton(value)),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::NoButton => bindings::MOUSE_NOBUTTON,
            MouseButton::Button1 => bindings::MOUSE_BUTTON1,
            MouseButton::Button2 => bindings::MOUSE_BUTTON2,
            MouseButton::Button3 => bindings::MOUSE_BUTTON3,
            MouseButton::Button4 => bindings::MOUSE_BUTTON4,
            MouseButton::Button5 => bindings::MOUSE_BUTTON5,
        }
    }
}

/// Represents a mouse event.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    /// The type of the mouse event.
    pub event_type: MouseEventType,
    /// The mouse button involved in the event.
    pub button: MouseButton,
    /// The number of clicks (usually 1 for single-click, 2 for double-click, etc.).
    pub clicks: u16,
    /// The x-coordinate of the mouse cursor.
    pub x: i16,
    /// The y-coordinate of the mouse cursor.
    pub y: i16,
}

impl From<&bindings::mouse_event_data> for MouseEvent {
    fn from(event: &bindings::mouse_event_data) -> Self {
        MouseEvent {
            event_type: MouseEventType::Moved, // This will be set correctly by the caller
            button: MouseButton::try_from(event.button as u32).unwrap_or(MouseButton::NoButton),
            clicks: event.clicks,
            x: event.x,
            y: event.y,
        }
    }
}

/// Simulates a mouse button press.
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `button` - The mouse button to press.
/// * `x` - The x-coordinate for the mouse event.
/// * `y` - The y-coordinate for the mouse event.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, mouse::{mouse_press, MouseButton}};
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
/// mouse_press(&hook, MouseButton::Button1, 100, 100).expect("Failed to press mouse button");
/// ```
pub fn mouse_press(uiohook: &Uiohook, button: MouseButton, x: i16, y: i16) -> Result<(), UiohookError> {
    let event = create_mouse_event(MouseEventType::Pressed, button, 1, x, y);
    uiohook.post_event(&crate::UiohookEvent::Mouse(event))
}

/// Simulates a mouse button release.
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `button` - The mouse button to release.
/// * `x` - The x-coordinate for the mouse event.
/// * `y` - The y-coordinate for the mouse event.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, mouse::{mouse_release, MouseButton}};
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
/// mouse_release(&hook, MouseButton::Button1, 100, 100).expect("Failed to release mouse button");
/// ```
pub fn mouse_release(uiohook: &Uiohook, button: MouseButton, x: i16, y: i16) -> Result<(), UiohookError> {
    let event = create_mouse_event(MouseEventType::Released, button, 1, x, y);
    uiohook.post_event(&crate::UiohookEvent::Mouse(event))
}

/// Simulates a mouse click (press and release).
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `button` - The mouse button to click.
/// * `x` - The x-coordinate for the mouse event.
/// * `y` - The y-coordinate for the mouse event.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, mouse::{mouse_click, MouseButton}};
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
/// mouse_click(&hook, MouseButton::Button1, 100, 100).expect("Failed to click mouse button");
/// ```
pub fn mouse_click(uiohook: &Uiohook, button: MouseButton, x: i16, y: i16) -> Result<(), UiohookError> {
    mouse_press(uiohook, button, x, y)?;
    mouse_release(uiohook, button, x, y)
}

/// Simulates moving the mouse cursor to a specific position.
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `x` - The x-coordinate to move the mouse to.
/// * `y` - The y-coordinate to move the mouse to.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, mouse::mouse_move};
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
/// mouse_move(&hook, 200, 200).expect("Failed to move mouse");
/// ```
pub fn mouse_move(uiohook: &Uiohook, x: i16, y: i16) -> Result<(), UiohookError> {
    let event = create_mouse_event(MouseEventType::Moved, MouseButton::NoButton, 0, x, y);
    uiohook.post_event(&crate::UiohookEvent::Mouse(event))
}

// Helper function to create a MouseEvent
fn create_mouse_event(event_type: MouseEventType, button: MouseButton, clicks: u16, x: i16, y: i16) -> MouseEvent {
    MouseEvent {
        event_type,
        button,
        clicks,
        x,
        y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_button_conversion() {
        assert_eq!(MouseButton::try_from(bindings::MOUSE_BUTTON1), Ok(MouseButton::Button1));
        assert_eq!(MouseButton::try_from(bindings::MOUSE_BUTTON2), Ok(MouseButton::Button2));
        assert_eq!(MouseButton::try_from(0xFFFF), Err(UiohookError::UnknownMouseButton(0xFFFF)));

        assert_eq!(u32::from(MouseButton::Button1), bindings::MOUSE_BUTTON1);
        assert_eq!(u32::from(MouseButton::Button2), bindings::MOUSE_BUTTON2);
    }

    #[test]
    fn test_mouse_event_from_bindings() {
        let binding_event = bindings::mouse_event_data {
            button: bindings::MOUSE_BUTTON1 as u16,
            clicks: 1,
            x: 100,
            y: 200,
        };

        let event = MouseEvent::from(&binding_event);
        assert_eq!(event.button, MouseButton::Button1);
        assert_eq!(event.clicks, 1);
        assert_eq!(event.x, 100);
        assert_eq!(event.y, 200);
    }

    // Add more tests as needed
}