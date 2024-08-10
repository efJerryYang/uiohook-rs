use colored::*;
use std::io::stdin;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use termios::{Termios, ECHO, ICANON, TCSANOW};
use uiohook_rs::{run, set_dispatch_proc, stop, UiohookEvent, keyboard, mouse};

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

    set_dispatch_proc(move |event: &UiohookEvent| {
        if let Some(keyboard_event) = keyboard::handle_keyboard_event(event) {
            match keyboard_event.event_type {
                keyboard::KeyboardEventType::Pressed | keyboard::KeyboardEventType::Released => {
                    let event_type = match keyboard_event.event_type {
                        keyboard::KeyboardEventType::Pressed => format!("{:<8}", "PRESSED").green(),
                        keyboard::KeyboardEventType::Released => format!("{:<8}", "RELEASED").red(),
                        _ => unreachable!(),
                    };

                    let key_info = if let Some(key_name) = keyboard_event.key_name {
                        format!("{:17}", key_name).yellow()
                    } else {
                        format!("Unknown Key (Code: {:<5})", keyboard_event.key_code).yellow()
                    };

                    println!(
                        "{} | {} | Code: {:<5} | Raw: {:<5}",
                        event_type, key_info, keyboard_event.key_code, keyboard_event.key_raw
                    );
                }
                keyboard::KeyboardEventType::Typed => {
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
                            keyboard_event.key_raw
                        );
                    }
                }
            }
        }

        if let Some(mouse_event) = mouse::handle_mouse_event(event) {
            let event_type = match mouse_event.event_type {
                mouse::MouseEventType::Moved => format!("{:<8}", "MOVED").yellow(),
                mouse::MouseEventType::Pressed => format!("{:<8}", "PRESSED").green(),
                mouse::MouseEventType::Released => format!("{:<8}", "RELEASED").red(),
                mouse::MouseEventType::Clicked => format!("{:<8}", "CLICKED").blue(),
                mouse::MouseEventType::Dragged => format!("{:<8}", "DRAGGED").magenta(),
                mouse::MouseEventType::Wheel => format!("{:<8}", "WHEEL").cyan(),
            };

            let details = match mouse_event.event_type {
                mouse::MouseEventType::Wheel => format!(
                    "Amount: {:<4} | Rotation: {:<4} | Direction: {:<4}",
                    mouse_event.amount, mouse_event.rotation, mouse_event.direction
                ),
                _ => format!(
                    "Button: {:<4} | Clicks: {:<4}",
                    mouse_event.button, mouse_event.clicks
                ),
            };

            println!(
                "{} | {:<17} | X: {:<5} | Y: {:<5} | {}",
                event_type,
                "Mouse".yellow(),
                mouse_event.x,
                mouse_event.y,
                details
            );
        }
    });

    let hook_thread = thread::spawn(move || {
        if let Err(e) = run() {
            eprintln!("Failed to run uiohook: {}", e);
        }
    });

    // Monitor the running flag in the main thread
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    // Stop uiohook
    if let Err(e) = stop() {
        eprintln!("Failed to stop uiohook: {}", e);
    }

    // Wait for the hook thread to finish
    hook_thread.join().unwrap();

    // Restore original terminal settings
    termios::tcsetattr(stdin_fd, TCSANOW, &original_termios).unwrap();

    println!("Exiting...");
}
