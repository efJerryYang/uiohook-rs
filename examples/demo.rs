use colored::*;
use std::io::stdin;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termios::{Termios, ECHO, ICANON, TCSANOW};
use uiohook_rs::{Uiohook, EventHandler, UiohookEvent};
use uiohook_rs::keyboard::{KeyboardEvent, KeyboardEventType, KeyCode};
use uiohook_rs::mouse::{MouseEvent, MouseEventType, MouseButton};
use uiohook_rs::wheel::WheelEvent;

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
                println!("{}", "Hook Enabled".green());
            }
            UiohookEvent::HookDisabled => {
                println!("{}", "Hook Disabled".red());
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
                    KeyboardEventType::Pressed => format!("{:<8}", "PRESSED").green(),
                    KeyboardEventType::Released => format!("{:<8}", "RELEASED").red(),
                    _ => unreachable!(),
                };

                let key_info = format!("{:?}", keyboard_event.key_code).yellow();

                println!(
                    "{} | {} | Code: {:<5} | Raw: {:<5}",
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
                        "{} | {:<17} | Code: {:<5} | Raw: {:<5}",
                        format!("{:<8}", "TYPED").blue(),
                        char_display.cyan(),
                        ch as u32,
                        keyboard_event.raw_code
                    );
                }
            }
        }
    }

    fn handle_mouse_event(&self, mouse_event: &MouseEvent) {
        let event_type = match mouse_event.event_type {
            MouseEventType::Moved => format!("{:<8}", "MOVED").yellow(),
            MouseEventType::Pressed => format!("{:<8}", "PRESSED").green(),
            MouseEventType::Released => format!("{:<8}", "RELEASED").red(),
            MouseEventType::Clicked => format!("{:<8}", "CLICKED").blue(),
            MouseEventType::Dragged => format!("{:<8}", "DRAGGED").magenta(),
        };

        let details = format!(
            "Button: {:<4} | Clicks: {:<4}",
            format!("{:?}", mouse_event.button),
            mouse_event.clicks
        );

        println!(
            "{} | {:<17} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            "Mouse".yellow(),
            mouse_event.x,
            mouse_event.y,
            details
        );
    }

    fn handle_wheel_event(&self, wheel_event: &WheelEvent) {
        let event_type = format!("{:<8}", "WHEEL").cyan();

        let details = format!(
            "Amount: {:<4} | Rotation: {:<4} | Direction: {:<4}",
            wheel_event.amount,
            wheel_event.rotation,
            if wheel_event.direction == uiohook_rs::wheel::WHEEL_VERTICAL_DIRECTION {
                "Vertical"
            } else {
                "Horizontal"
            }
        );

        println!(
            "{} | {:<17} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            "Mouse Wheel".yellow(),
            wheel_event.x,
            wheel_event.y,
            details
        );
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Disable terminal echo to provide a cleaner output
    let stdin_fd = stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original_termios = termios.clone();
    termios.c_lflag &= !(ECHO | ICANON);
    termios::tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();

    println!("Press Ctrl-C to exit");

    // Set up Ctrl-C handler
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let event_handler = DemoEventHandler {
        running: running.clone(),
    };

    let uiohook = Uiohook::new(event_handler);

    let hook_thread = thread::spawn(move || {
        if let Err(e) = uiohook.run() {
            eprintln!("Failed to run uiohook: {}", e);
        }
    });

    // Monitor the running flag in the main thread
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    // Stop uiohook
    if let Err(e) = Uiohook::stop() {
        eprintln!("Failed to stop uiohook: {}", e);
    }

    // Wait for the hook thread to finish
    hook_thread.join().unwrap();

    // Restore original terminal settings
    termios::tcsetattr(stdin_fd, TCSANOW, &original_termios).unwrap();

    println!("Exiting...");
}