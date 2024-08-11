use uiohook_rs::hook::keyboard::{KeyboardEvent, KeyboardEventType};
use uiohook_rs::hook::mouse::{MouseEvent, MouseEventType};
use uiohook_rs::hook::wheel::WheelEvent;
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoHandler;

impl EventHandler for DemoHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        match event {
            UiohookEvent::Keyboard(keyboard_event) => {
                self.handle_keyboard_event(keyboard_event);
            }
            UiohookEvent::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event);
            }
            UiohookEvent::Wheel(wheel_event) => {
                self.handle_wheel_event(wheel_event);
            }
            _ => {}
        }
    }
}

impl DemoHandler {
    fn handle_keyboard_event(&self, keyboard_event: &KeyboardEvent) {
        match keyboard_event.event_type {
            KeyboardEventType::Pressed | KeyboardEventType::Released => {
                let event_type = match keyboard_event.event_type {
                    KeyboardEventType::Pressed => "PRESSED",
                    KeyboardEventType::Released => "RELEASED",
                    _ => unreachable!(),
                };

                let key_info = format!("{:?}", keyboard_event.key_code);

                println!(
                    "{:<8} | {:<17} | Code: {:<5} | Raw: {:<5}",
                    event_type,
                    key_info,
                    keyboard_event.key_code as u16,
                    keyboard_event.raw_code
                );
            }
            KeyboardEventType::Typed => {
                if let Some(ch) = keyboard_event.key_char {
                    let char_display = if ch.is_control() {
                        format!("(Control-{:02X})", ch as u8)
                    } else {
                        ch.to_string()
                    };

                    println!(
                        "{:<8} | {:<17} | Code: {:<5} | Raw: {:<5}",
                        "TYPED",
                        char_display,
                        ch as u32,
                        keyboard_event.raw_code
                    );
                }
            }
        }
    }

    fn handle_mouse_event(&self, mouse_event: &MouseEvent) {
        let event_type = match mouse_event.event_type {
            MouseEventType::Moved => "MOVED",
            MouseEventType::Pressed => "PRESSED",
            MouseEventType::Released => "RELEASED",
            MouseEventType::Clicked => "CLICKED",
            MouseEventType::Dragged => "DRAGGED",
        };

        let details = format!(
            "Button: {:<8} | Clicks: {:<4}",
            format!("{:?}", mouse_event.button),
            mouse_event.clicks
        );

        println!(
            "{:<8} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            mouse_event.x,
            mouse_event.y,
            details
        );
    }

    fn handle_wheel_event(&self, wheel_event: &WheelEvent) {
        let event_type = "SCROLL";

        let details = format!(
            "Amount: {:<4} | Rotation: {:<4} | Direction: {:<9}",
            wheel_event.amount,
            wheel_event.rotation,
            if wheel_event.direction == uiohook_rs::hook::wheel::WHEEL_VERTICAL_DIRECTION {
                "Vertical"
            } else {
                "Horizontal"
            }
        );

        println!(
            "{:<8} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            wheel_event.x,
            wheel_event.y,
            details
        );
    }
}

fn main() {
    println!("Running... Press Ctrl-C to exit");

    let event_handler = DemoHandler;

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
    }

    println!("Exiting...");
}
