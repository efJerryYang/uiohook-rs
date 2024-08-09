use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uiohook_rs::{set_dispatch_proc, run, stop, UiohookEvent};

mod keyboard {
    use uiohook_rs::{UiohookEvent, event_type};
    use std::char;

    pub struct KeyboardEvent {
        pub key_code: u16,
        pub key_char: Option<char>,
        pub key_raw: u16,
    }

    impl From<&UiohookEvent> for KeyboardEvent {
        fn from(event: &UiohookEvent) -> Self {
            let keyboard = event.keyboard_event().unwrap();
            KeyboardEvent {
                key_code: keyboard.keycode,
                key_char: char::from_u32(keyboard.keychar as u32),
                key_raw: keyboard.rawcode,
            }
        }
    }

    pub fn handle_keyboard_event(event: &UiohookEvent) -> Option<KeyboardEvent> {
        match event.event_type() {
            event_type::EVENT_KEY_PRESSED | event_type::EVENT_KEY_RELEASED => {
                Some(KeyboardEvent::from(event))
            }
            _ => None,
        }
    }
}

mod mouse {
    use uiohook_rs::{UiohookEvent, event_type};

    pub enum MouseEventType {
        Moved,
        Pressed,
        Released,
        Clicked,
        Dragged,
        Wheel,
    }

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

    impl From<&UiohookEvent> for MouseEvent {
        fn from(event: &UiohookEvent) -> Self {
            match event.event_type() {
                event_type::EVENT_MOUSE_WHEEL => {
                    let wheel = event.wheel_event().unwrap();
                    MouseEvent {
                        event_type: MouseEventType::Wheel,
                        x: wheel.x,
                        y: wheel.y,
                        button: 0,
                        clicks: 0,
                        amount: wheel.amount,
                        rotation: wheel.rotation,
                        direction: wheel.direction,
                    }
                }
                _ => {
                    let mouse = event.mouse_event().unwrap();
                    let event_type = match event.event_type() {
                        event_type::EVENT_MOUSE_MOVED => MouseEventType::Moved,
                        event_type::EVENT_MOUSE_PRESSED => MouseEventType::Pressed,
                        event_type::EVENT_MOUSE_RELEASED => MouseEventType::Released,
                        event_type::EVENT_MOUSE_CLICKED => MouseEventType::Clicked,
                        event_type::EVENT_MOUSE_DRAGGED => MouseEventType::Dragged,
                        _ => unreachable!(),
                    };
                    MouseEvent {
                        event_type,
                        x: mouse.x,
                        y: mouse.y,
                        button: mouse.button,
                        clicks: mouse.clicks,
                        amount: 0,
                        rotation: 0,
                        direction: 0,
                    }
                }
            }
        }
    }

    pub fn handle_mouse_event(event: &UiohookEvent) -> Option<MouseEvent> {
        match event.event_type() {
            event_type::EVENT_MOUSE_MOVED |
            event_type::EVENT_MOUSE_PRESSED |
            event_type::EVENT_MOUSE_RELEASED |
            event_type::EVENT_MOUSE_CLICKED |
            event_type::EVENT_MOUSE_DRAGGED |
            event_type::EVENT_MOUSE_WHEEL => Some(MouseEvent::from(event)),
            _ => None,
        }
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    println!("Press Ctrl-C to exit");

    set_dispatch_proc(move |event: &UiohookEvent| {
        if let Some(keyboard_event) = keyboard::handle_keyboard_event(event) {
            println!("Keyboard event: Key code: {}, Char: {}, Raw: {}",
                keyboard_event.key_code,
                keyboard_event.key_char.map_or("None".to_string(), |c| c.to_string()),
                keyboard_event.key_raw,
            );
        }

        if let Some(mouse_event) = mouse::handle_mouse_event(event) {
            match mouse_event.event_type {
                mouse::MouseEventType::Moved => {
                    println!("Mouse moved to ({}, {})", mouse_event.x, mouse_event.y);
                }
                mouse::MouseEventType::Pressed => {
                    println!("Mouse button {} pressed at ({}, {})", mouse_event.button, mouse_event.x, mouse_event.y);
                }
                mouse::MouseEventType::Released => {
                    println!("Mouse button {} released at ({}, {})", mouse_event.button, mouse_event.x, mouse_event.y);
                }
                mouse::MouseEventType::Clicked => {
                    println!("Mouse clicked {} times at ({}, {})", mouse_event.clicks, mouse_event.x, mouse_event.y);
                }
                mouse::MouseEventType::Dragged => {
                    println!("Mouse dragged to ({}, {})", mouse_event.x, mouse_event.y);
                }
                mouse::MouseEventType::Wheel => {
                    println!("Mouse wheel scrolled: amount {}, rotation {}, direction {}",
                        mouse_event.amount, mouse_event.rotation, mouse_event.direction
                    );
                }
            }
        }
    });

    if let Err(e) = run() {
        eprintln!("Failed to run uiohook: {}", e);
        return;
    }

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    if let Err(e) = stop() {
        eprintln!("Failed to stop uiohook: {}", e);
    }
}