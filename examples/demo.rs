use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uiohook_rs::hook::keyboard::{KeyboardEvent, KeyboardEventType};
use uiohook_rs::hook::mouse::{MouseEvent, MouseEventType};
use uiohook_rs::hook::wheel::WheelEvent;
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoEventHandler {
    running: Arc<AtomicBool>,
}

impl EventHandler for DemoEventHandler {
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
            UiohookEvent::HookEnabled => {
                println!("Hook Enabled");
            }
            UiohookEvent::HookDisabled => {
                println!("Hook Disabled");
                self.running.store(false, Ordering::SeqCst);
            }
        }
    }
}

impl DemoEventHandler {
    fn handle_keyboard_event(&self, keyboard_event: &KeyboardEvent) {
        match keyboard_event.event_type {
            KeyboardEventType::Pressed | KeyboardEventType::Released => {
                let event_type = match keyboard_event.event_type {
                    KeyboardEventType::Pressed => format!("{:<8}", "PRESSED"),
                    KeyboardEventType::Released => format!("{:<8}", "RELEASED"),
                    _ => unreachable!(),
                };

                let key_info = format!("{:?}", keyboard_event.key_code);

                println!(
                    "{} | {:<17} | Code: {:<5} | Raw: {:<5}",
                    event_type, key_info, keyboard_event.key_code as u16, keyboard_event.raw_code
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
                        "{} | {:<17} | Code: {:<5} | Raw: {:<5}",
                        format!("{:<8}", "TYPED"),
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
            MouseEventType::Moved => format!("{:<8}", "MOVED"),
            MouseEventType::Pressed => format!("{:<8}", "PRESSED"),
            MouseEventType::Released => format!("{:<8}", "RELEASED"),
            MouseEventType::Clicked => format!("{:<8}", "CLICKED"),
            MouseEventType::Dragged => format!("{:<8}", "DRAGGED"),
        };

        let details = format!(
            "Button: {:<8} | Clicks: {:<4}",
            format!("{:?}", mouse_event.button),
            mouse_event.clicks
        );

        println!(
            "{} | {:<17} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            "Mouse",
            mouse_event.x,
            mouse_event.y,
            details
        );
    }

    fn handle_wheel_event(&self, wheel_event: &WheelEvent) {
        let event_type = format!("{:<8}", "SCROLL");

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
            "{} | {:<17} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            "Mouse Wheel",
            wheel_event.x,
            wheel_event.y,
            details
        );
    }
}

fn main() {
    #[cfg(target_os = "macos")]
    use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopStop, CFRunLoopRun};

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    println!("Press Ctrl-C to exit");

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        #[cfg(target_os = "macos")]
        {
            println!("Ctrl-C pressed, stopping CFRunLoop...");
            unsafe {
                CFRunLoopStop(CFRunLoopGetMain());
            }
        }
    })
    .expect("Error setting Ctrl-C handler");

    let event_handler = DemoEventHandler {
        running: running.clone(),
    };

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
        return;
    }

    #[cfg(target_os = "macos")]
    {
        println!("Starting CFRunLoopRun on macOS main thread...");
        unsafe {
            CFRunLoopRun();
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        while running.load(Ordering::SeqCst) {
            std::thread::sleep(time::Duration::from_millis(100));
        }
    }

    // Stop uiohook
    if let Err(e) = uiohook.stop() {
        eprintln!("Failed to stop uiohook: {}", e);
    }

    println!("Exiting...");
}
