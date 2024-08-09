use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uiohook_rs::{event_type, run, set_dispatch_proc, stop, UiohookEvent};
use colored::*;
use termios::{Termios, ECHO, ICANON, TCSANOW};
use std::io::stdin;
use std::os::unix::io::AsRawFd;
mod keyboard {
    use uiohook_rs::{event_type, UiohookEvent};

    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref KEY_NAMES: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            // "[\w/\s]{1,17}" is the regex pattern for key names
            m.insert(uiohook_rs::VC_ESCAPE, "Escape");
            m.insert(uiohook_rs::VC_F1, "F1");
            m.insert(uiohook_rs::VC_F2, "F2");
            m.insert(uiohook_rs::VC_F3, "F3");
            m.insert(uiohook_rs::VC_F4, "F4");
            m.insert(uiohook_rs::VC_F5, "F5");
            m.insert(uiohook_rs::VC_F6, "F6");
            m.insert(uiohook_rs::VC_F7, "F7");
            m.insert(uiohook_rs::VC_F8, "F8");
            m.insert(uiohook_rs::VC_F9, "F9");
            m.insert(uiohook_rs::VC_F10, "F10");
            m.insert(uiohook_rs::VC_F11, "F11");
            m.insert(uiohook_rs::VC_F12, "F12");
            m.insert(uiohook_rs::VC_F13, "F13");
            m.insert(uiohook_rs::VC_F14, "F14");
            m.insert(uiohook_rs::VC_F15, "F15");
            m.insert(uiohook_rs::VC_F16, "F16");
            m.insert(uiohook_rs::VC_F17, "F17");
            m.insert(uiohook_rs::VC_F18, "F18");
            m.insert(uiohook_rs::VC_F19, "F19");
            m.insert(uiohook_rs::VC_F20, "F20");
            m.insert(uiohook_rs::VC_F21, "F21");
            m.insert(uiohook_rs::VC_F22, "F22");
            m.insert(uiohook_rs::VC_F23, "F23");
            m.insert(uiohook_rs::VC_F24, "F24");
            m.insert(uiohook_rs::VC_BACKQUOTE, "Backquote");
            m.insert(uiohook_rs::VC_1, "1");
            m.insert(uiohook_rs::VC_2, "2");
            m.insert(uiohook_rs::VC_3, "3");
            m.insert(uiohook_rs::VC_4, "4");
            m.insert(uiohook_rs::VC_5, "5");
            m.insert(uiohook_rs::VC_6, "6");
            m.insert(uiohook_rs::VC_7, "7");
            m.insert(uiohook_rs::VC_8, "8");
            m.insert(uiohook_rs::VC_9, "9");
            m.insert(uiohook_rs::VC_0, "0");
            m.insert(uiohook_rs::VC_MINUS, "Minus");
            m.insert(uiohook_rs::VC_EQUALS, "Equals");
            m.insert(uiohook_rs::VC_BACKSPACE, "Backspace");
            m.insert(uiohook_rs::VC_TAB, "Tab");
            m.insert(uiohook_rs::VC_CAPS_LOCK, "Caps Lock");
            m.insert(uiohook_rs::VC_A, "A");
            m.insert(uiohook_rs::VC_B, "B");
            m.insert(uiohook_rs::VC_C, "C");
            m.insert(uiohook_rs::VC_D, "D");
            m.insert(uiohook_rs::VC_E, "E");
            m.insert(uiohook_rs::VC_F, "F");
            m.insert(uiohook_rs::VC_G, "G");
            m.insert(uiohook_rs::VC_H, "H");
            m.insert(uiohook_rs::VC_I, "I");
            m.insert(uiohook_rs::VC_J, "J");
            m.insert(uiohook_rs::VC_K, "K");
            m.insert(uiohook_rs::VC_L, "L");
            m.insert(uiohook_rs::VC_M, "M");
            m.insert(uiohook_rs::VC_N, "N");
            m.insert(uiohook_rs::VC_O, "O");
            m.insert(uiohook_rs::VC_P, "P");
            m.insert(uiohook_rs::VC_Q, "Q");
            m.insert(uiohook_rs::VC_R, "R");
            m.insert(uiohook_rs::VC_S, "S");
            m.insert(uiohook_rs::VC_T, "T");
            m.insert(uiohook_rs::VC_U, "U");
            m.insert(uiohook_rs::VC_V, "V");
            m.insert(uiohook_rs::VC_W, "W");
            m.insert(uiohook_rs::VC_X, "X");
            m.insert(uiohook_rs::VC_Y, "Y");
            m.insert(uiohook_rs::VC_Z, "Z");
            m.insert(uiohook_rs::VC_OPEN_BRACKET, "Open Bracket");
            m.insert(uiohook_rs::VC_CLOSE_BRACKET, "Close Bracket");
            m.insert(uiohook_rs::VC_BACK_SLASH, "Backslash");
            m.insert(uiohook_rs::VC_SEMICOLON, "Semicolon");
            m.insert(uiohook_rs::VC_QUOTE, "Quote");
            m.insert(uiohook_rs::VC_ENTER, "Enter");
            m.insert(uiohook_rs::VC_COMMA, "Comma");
            m.insert(uiohook_rs::VC_PERIOD, "Period");
            m.insert(uiohook_rs::VC_SLASH, "Slash");
            m.insert(uiohook_rs::VC_SPACE, "Space");
            m.insert(uiohook_rs::VC_PRINTSCREEN, "Print Screen");
            m.insert(uiohook_rs::VC_SCROLL_LOCK, "Scroll Lock");
            m.insert(uiohook_rs::VC_PAUSE, "Pause");
            m.insert(uiohook_rs::VC_LESSER_GREATER, "Less/Greater");
            m.insert(uiohook_rs::VC_INSERT, "Insert");
            m.insert(uiohook_rs::VC_DELETE, "Delete");
            m.insert(uiohook_rs::VC_HOME, "Home");
            m.insert(uiohook_rs::VC_END, "End");
            m.insert(uiohook_rs::VC_PAGE_UP, "Page Up");
            m.insert(uiohook_rs::VC_PAGE_DOWN, "Page Down");
            m.insert(uiohook_rs::VC_UP, "Up");
            m.insert(uiohook_rs::VC_LEFT, "Left");
            m.insert(uiohook_rs::VC_CLEAR, "Clear");
            m.insert(uiohook_rs::VC_RIGHT, "Right");
            m.insert(uiohook_rs::VC_DOWN, "Down");
            m.insert(uiohook_rs::VC_NUM_LOCK, "Num Lock");
            m.insert(uiohook_rs::VC_KP_DIVIDE, "Keypad Divide");
            m.insert(uiohook_rs::VC_KP_MULTIPLY, "Keypad Multiply");
            m.insert(uiohook_rs::VC_KP_SUBTRACT, "Keypad Subtract");
            m.insert(uiohook_rs::VC_KP_EQUALS, "Keypad Equals");
            m.insert(uiohook_rs::VC_KP_ADD, "Keypad Add");
            m.insert(uiohook_rs::VC_KP_ENTER, "Keypad Enter");
            m.insert(uiohook_rs::VC_KP_SEPARATOR, "Keypad Separator");
            m.insert(uiohook_rs::VC_KP_1, "Keypad 1");
            m.insert(uiohook_rs::VC_KP_2, "Keypad 2");
            m.insert(uiohook_rs::VC_KP_3, "Keypad 3");
            m.insert(uiohook_rs::VC_KP_4, "Keypad 4");
            m.insert(uiohook_rs::VC_KP_5, "Keypad 5");
            m.insert(uiohook_rs::VC_KP_6, "Keypad 6");
            m.insert(uiohook_rs::VC_KP_7, "Keypad 7");
            m.insert(uiohook_rs::VC_KP_8, "Keypad 8");
            m.insert(uiohook_rs::VC_KP_9, "Keypad 9");
            m.insert(uiohook_rs::VC_KP_0, "Keypad 0");
            m.insert(uiohook_rs::VC_KP_END, "Keypad End");
            m.insert(uiohook_rs::VC_KP_DOWN, "Keypad Down");
            m.insert(uiohook_rs::VC_KP_PAGE_DOWN, "Keypad Page Down");
            m.insert(uiohook_rs::VC_KP_LEFT, "Keypad Left");
            m.insert(uiohook_rs::VC_KP_CLEAR, "Keypad Clear");
            m.insert(uiohook_rs::VC_KP_RIGHT, "Keypad Right");
            m.insert(uiohook_rs::VC_KP_HOME, "Keypad Home");
            m.insert(uiohook_rs::VC_KP_UP, "Keypad Up");
            m.insert(uiohook_rs::VC_KP_PAGE_UP, "Keypad Page Up");
            m.insert(uiohook_rs::VC_KP_INSERT, "Keypad Insert");
            m.insert(uiohook_rs::VC_KP_DELETE, "Keypad Delete");
            m.insert(uiohook_rs::VC_SHIFT_L, "Left Shift");
            m.insert(uiohook_rs::VC_SHIFT_R, "Right Shift");
            m.insert(uiohook_rs::VC_CONTROL_L, "Left Control");
            m.insert(uiohook_rs::VC_CONTROL_R, "Right Control");
            m.insert(uiohook_rs::VC_ALT_L, "Left Alt");
            m.insert(uiohook_rs::VC_ALT_R, "Right Alt");
            m.insert(uiohook_rs::VC_META_L, "Left Meta");
            m.insert(uiohook_rs::VC_META_R, "Right Meta");
            m.insert(uiohook_rs::VC_CONTEXT_MENU, "Context Menu");
            m.insert(uiohook_rs::VC_POWER, "Power");
            m.insert(uiohook_rs::VC_SLEEP, "Sleep");
            m.insert(uiohook_rs::VC_WAKE, "Wake");
            m.insert(uiohook_rs::VC_MEDIA_PLAY, "Media Play");
            m.insert(uiohook_rs::VC_MEDIA_STOP, "Media Stop");
            m.insert(uiohook_rs::VC_MEDIA_PREVIOUS, "Media Previous");
            m.insert(uiohook_rs::VC_MEDIA_NEXT, "Media Next");
            m.insert(uiohook_rs::VC_MEDIA_SELECT, "Media Select");
            m.insert(uiohook_rs::VC_MEDIA_EJECT, "Media Eject");
            m.insert(uiohook_rs::VC_VOLUME_MUTE, "Volume Mute");
            m.insert(uiohook_rs::VC_VOLUME_UP, "Volume Up");
            m.insert(uiohook_rs::VC_VOLUME_DOWN, "Volume Down");
            m.insert(uiohook_rs::VC_APP_MAIL, "App Mail");
            m.insert(uiohook_rs::VC_APP_CALCULATOR, "App Calculator");
            m.insert(uiohook_rs::VC_APP_MUSIC, "App Music");
            m.insert(uiohook_rs::VC_APP_PICTURES, "App Pictures");
            m.insert(uiohook_rs::VC_BROWSER_SEARCH, "Browser Search");
            m.insert(uiohook_rs::VC_BROWSER_HOME, "Browser Home");
            m.insert(uiohook_rs::VC_BROWSER_BACK, "Browser Back");
            m.insert(uiohook_rs::VC_BROWSER_FORWARD, "Browser Forward");
            m.insert(uiohook_rs::VC_BROWSER_STOP, "Browser Stop");
            m.insert(uiohook_rs::VC_BROWSER_REFRESH, "Browser Refresh");
            m.insert(uiohook_rs::VC_BROWSER_FAVORITES, "Browser Favorites");
            m.insert(uiohook_rs::VC_KATAKANA, "Katakana");
            m.insert(uiohook_rs::VC_UNDERSCORE, "Underscore");
            m.insert(uiohook_rs::VC_FURIGANA, "Furigana");
            m.insert(uiohook_rs::VC_KANJI, "Kanji");
            m.insert(uiohook_rs::VC_HIRAGANA, "Hiragana");
            m.insert(uiohook_rs::VC_YEN, "Yen");
            m.insert(uiohook_rs::VC_KP_COMMA, "Keypad Comma");
            m.insert(uiohook_rs::VC_SUN_HELP, "Sun Help");
            m.insert(uiohook_rs::VC_SUN_STOP, "Sun Stop");
            m.insert(uiohook_rs::VC_SUN_PROPS, "Sun Props");
            m.insert(uiohook_rs::VC_SUN_FRONT, "Sun Front");
            m.insert(uiohook_rs::VC_SUN_OPEN, "Sun Open");
            m.insert(uiohook_rs::VC_SUN_FIND, "Sun Find");
            m.insert(uiohook_rs::VC_SUN_AGAIN, "Sun Again");
            m.insert(uiohook_rs::VC_SUN_UNDO, "Sun Undo");
            m.insert(uiohook_rs::VC_SUN_COPY, "Sun Copy");
            m.insert(uiohook_rs::VC_SUN_INSERT, "Sun Insert");
            m.insert(uiohook_rs::VC_SUN_CUT, "Sun Cut");
            m.insert(uiohook_rs::VC_UNDEFINED, "Undefined");
            m.insert(uiohook_rs::CHAR_UNDEFINED, "Undefined");

            m
        };
    }

    pub enum KeyboardEventType {
        Pressed,
        Released,
        Typed,
    }

    pub struct KeyboardEvent {
        pub event_type: KeyboardEventType,
        pub key_code: u16,
        pub key_raw: u16,
        pub key_char: Option<char>,
        pub key_name: Option<&'static str>,
    }

    impl From<&UiohookEvent> for KeyboardEvent {
        fn from(event: &UiohookEvent) -> Self {
            let keyboard = event.keyboard_event().unwrap();
            let key_name = KEY_NAMES.get(&(keyboard.keycode as u32)).copied();

            match event.event_type() {
                event_type::EVENT_KEY_PRESSED => KeyboardEvent {
                    event_type: KeyboardEventType::Pressed,
                    key_code: keyboard.keycode,
                    key_raw: keyboard.rawcode,
                    key_char: None,
                    key_name,
                },
                event_type::EVENT_KEY_RELEASED => KeyboardEvent {
                    event_type: KeyboardEventType::Released,
                    key_code: keyboard.keycode,
                    key_raw: keyboard.rawcode,
                    key_char: None,
                    key_name,
                },
                event_type::EVENT_KEY_TYPED => KeyboardEvent {
                    event_type: KeyboardEventType::Typed,
                    key_code: keyboard.keycode,
                    key_raw: keyboard.rawcode,
                    key_char: char::from_u32(keyboard.keychar as u32),
                    key_name,
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn handle_keyboard_event(event: &UiohookEvent) -> Option<KeyboardEvent> {
        match event.event_type() {
            event_type::EVENT_KEY_PRESSED
            | event_type::EVENT_KEY_RELEASED
            | event_type::EVENT_KEY_TYPED => Some(KeyboardEvent::from(event)),
            _ => None,
        }
    }
}
mod mouse {
    use uiohook_rs::{event_type, UiohookEvent};

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
            event_type::EVENT_MOUSE_MOVED
            | event_type::EVENT_MOUSE_PRESSED
            | event_type::EVENT_MOUSE_RELEASED
            | event_type::EVENT_MOUSE_CLICKED
            | event_type::EVENT_MOUSE_DRAGGED
            | event_type::EVENT_MOUSE_WHEEL => Some(MouseEvent::from(event)),
            _ => None,
        }
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Disable terminal echo
    let stdin_fd = stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let original_termios = termios.clone();
    termios.c_lflag &= !(ECHO | ICANON);
    termios::tcsetattr(stdin_fd, TCSANOW, &termios).unwrap();

    println!("Press Ctrl-C to exit");

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
                        // format!("{}", key_name.yellow())
                        format!("{:17}", key_name).yellow()
                    } else {
                        // format!("Unknown Key (Code: {})", keyboard_event.key_code)
                        format!("Unknown Key (Code: {:<5})", keyboard_event.key_code).yellow()
                    };

                    println!(
                        "{} | {} | Code: {:<5} | Raw: {:<5}",
                        event_type,
                        key_info,
                        keyboard_event.key_code,
                        keyboard_event.key_raw
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

    termios::tcsetattr(stdin_fd, TCSANOW, &original_termios).unwrap();
}
