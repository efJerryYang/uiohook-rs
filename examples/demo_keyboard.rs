use uiohook_rs::hook::keyboard::{KeyboardEvent, KeyboardEventType};
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoKeyboardHandler;

impl EventHandler for DemoKeyboardHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        if let UiohookEvent::Keyboard(keyboard_event) = event {
            self.handle_keyboard_event(keyboard_event);
        }
    }
}

impl DemoKeyboardHandler {
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
}

fn main() {
    println!("Running... Press Ctrl-C to exit");

    let event_handler = DemoKeyboardHandler;

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
        return;
    }

    #[cfg(target_os = "macos")]
    {
        unsafe {
            core_foundation::runloop::CFRunLoopRun();
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    println!("Exiting...");
}
