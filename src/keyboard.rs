use crate::{event_type, UiohookEvent};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEY_NAMES: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        // "[\w/\s]{1,17}" is the regex pattern for key names
        m.insert(crate::VC_ESCAPE, "Escape");
        m.insert(crate::VC_F1, "F1");
        m.insert(crate::VC_F2, "F2");
        m.insert(crate::VC_F3, "F3");
        m.insert(crate::VC_F4, "F4");
        m.insert(crate::VC_F5, "F5");
        m.insert(crate::VC_F6, "F6");
        m.insert(crate::VC_F7, "F7");
        m.insert(crate::VC_F8, "F8");
        m.insert(crate::VC_F9, "F9");
        m.insert(crate::VC_F10, "F10");
        m.insert(crate::VC_F11, "F11");
        m.insert(crate::VC_F12, "F12");
        m.insert(crate::VC_F13, "F13");
        m.insert(crate::VC_F14, "F14");
        m.insert(crate::VC_F15, "F15");
        m.insert(crate::VC_F16, "F16");
        m.insert(crate::VC_F17, "F17");
        m.insert(crate::VC_F18, "F18");
        m.insert(crate::VC_F19, "F19");
        m.insert(crate::VC_F20, "F20");
        m.insert(crate::VC_F21, "F21");
        m.insert(crate::VC_F22, "F22");
        m.insert(crate::VC_F23, "F23");
        m.insert(crate::VC_F24, "F24");
        m.insert(crate::VC_BACKQUOTE, "Backquote");
        m.insert(crate::VC_1, "1");
        m.insert(crate::VC_2, "2");
        m.insert(crate::VC_3, "3");
        m.insert(crate::VC_4, "4");
        m.insert(crate::VC_5, "5");
        m.insert(crate::VC_6, "6");
        m.insert(crate::VC_7, "7");
        m.insert(crate::VC_8, "8");
        m.insert(crate::VC_9, "9");
        m.insert(crate::VC_0, "0");
        m.insert(crate::VC_MINUS, "Minus");
        m.insert(crate::VC_EQUALS, "Equals");
        m.insert(crate::VC_BACKSPACE, "Backspace");
        m.insert(crate::VC_TAB, "Tab");
        m.insert(crate::VC_CAPS_LOCK, "Caps Lock");
        m.insert(crate::VC_A, "A");
        m.insert(crate::VC_B, "B");
        m.insert(crate::VC_C, "C");
        m.insert(crate::VC_D, "D");
        m.insert(crate::VC_E, "E");
        m.insert(crate::VC_F, "F");
        m.insert(crate::VC_G, "G");
        m.insert(crate::VC_H, "H");
        m.insert(crate::VC_I, "I");
        m.insert(crate::VC_J, "J");
        m.insert(crate::VC_K, "K");
        m.insert(crate::VC_L, "L");
        m.insert(crate::VC_M, "M");
        m.insert(crate::VC_N, "N");
        m.insert(crate::VC_O, "O");
        m.insert(crate::VC_P, "P");
        m.insert(crate::VC_Q, "Q");
        m.insert(crate::VC_R, "R");
        m.insert(crate::VC_S, "S");
        m.insert(crate::VC_T, "T");
        m.insert(crate::VC_U, "U");
        m.insert(crate::VC_V, "V");
        m.insert(crate::VC_W, "W");
        m.insert(crate::VC_X, "X");
        m.insert(crate::VC_Y, "Y");
        m.insert(crate::VC_Z, "Z");
        m.insert(crate::VC_OPEN_BRACKET, "Open Bracket");
        m.insert(crate::VC_CLOSE_BRACKET, "Close Bracket");
        m.insert(crate::VC_BACK_SLASH, "Backslash");
        m.insert(crate::VC_SEMICOLON, "Semicolon");
        m.insert(crate::VC_QUOTE, "Quote");
        m.insert(crate::VC_ENTER, "Enter");
        m.insert(crate::VC_COMMA, "Comma");
        m.insert(crate::VC_PERIOD, "Period");
        m.insert(crate::VC_SLASH, "Slash");
        m.insert(crate::VC_SPACE, "Space");
        m.insert(crate::VC_PRINTSCREEN, "Print Screen");
        m.insert(crate::VC_SCROLL_LOCK, "Scroll Lock");
        m.insert(crate::VC_PAUSE, "Pause");
        m.insert(crate::VC_LESSER_GREATER, "Less/Greater");
        m.insert(crate::VC_INSERT, "Insert");
        m.insert(crate::VC_DELETE, "Delete");
        m.insert(crate::VC_HOME, "Home");
        m.insert(crate::VC_END, "End");
        m.insert(crate::VC_PAGE_UP, "Page Up");
        m.insert(crate::VC_PAGE_DOWN, "Page Down");
        m.insert(crate::VC_UP, "Up");
        m.insert(crate::VC_LEFT, "Left");
        m.insert(crate::VC_CLEAR, "Clear");
        m.insert(crate::VC_RIGHT, "Right");
        m.insert(crate::VC_DOWN, "Down");
        m.insert(crate::VC_NUM_LOCK, "Num Lock");
        m.insert(crate::VC_KP_DIVIDE, "Keypad Divide");
        m.insert(crate::VC_KP_MULTIPLY, "Keypad Multiply");
        m.insert(crate::VC_KP_SUBTRACT, "Keypad Subtract");
        m.insert(crate::VC_KP_EQUALS, "Keypad Equals");
        m.insert(crate::VC_KP_ADD, "Keypad Add");
        m.insert(crate::VC_KP_ENTER, "Keypad Enter");
        m.insert(crate::VC_KP_SEPARATOR, "Keypad Separator");
        m.insert(crate::VC_KP_1, "Keypad 1");
        m.insert(crate::VC_KP_2, "Keypad 2");
        m.insert(crate::VC_KP_3, "Keypad 3");
        m.insert(crate::VC_KP_4, "Keypad 4");
        m.insert(crate::VC_KP_5, "Keypad 5");
        m.insert(crate::VC_KP_6, "Keypad 6");
        m.insert(crate::VC_KP_7, "Keypad 7");
        m.insert(crate::VC_KP_8, "Keypad 8");
        m.insert(crate::VC_KP_9, "Keypad 9");
        m.insert(crate::VC_KP_0, "Keypad 0");
        m.insert(crate::VC_KP_END, "Keypad End");
        m.insert(crate::VC_KP_DOWN, "Keypad Down");
        m.insert(crate::VC_KP_PAGE_DOWN, "Keypad Page Down");
        m.insert(crate::VC_KP_LEFT, "Keypad Left");
        m.insert(crate::VC_KP_CLEAR, "Keypad Clear");
        m.insert(crate::VC_KP_RIGHT, "Keypad Right");
        m.insert(crate::VC_KP_HOME, "Keypad Home");
        m.insert(crate::VC_KP_UP, "Keypad Up");
        m.insert(crate::VC_KP_PAGE_UP, "Keypad Page Up");
        m.insert(crate::VC_KP_INSERT, "Keypad Insert");
        m.insert(crate::VC_KP_DELETE, "Keypad Delete");
        m.insert(crate::VC_SHIFT_L, "Left Shift");
        m.insert(crate::VC_SHIFT_R, "Right Shift");
        m.insert(crate::VC_CONTROL_L, "Left Control");
        m.insert(crate::VC_CONTROL_R, "Right Control");
        m.insert(crate::VC_ALT_L, "Left Alt");
        m.insert(crate::VC_ALT_R, "Right Alt");
        m.insert(crate::VC_META_L, "Left Meta");
        m.insert(crate::VC_META_R, "Right Meta");
        m.insert(crate::VC_CONTEXT_MENU, "Context Menu");
        m.insert(crate::VC_POWER, "Power");
        m.insert(crate::VC_SLEEP, "Sleep");
        m.insert(crate::VC_WAKE, "Wake");
        m.insert(crate::VC_MEDIA_PLAY, "Media Play");
        m.insert(crate::VC_MEDIA_STOP, "Media Stop");
        m.insert(crate::VC_MEDIA_PREVIOUS, "Media Previous");
        m.insert(crate::VC_MEDIA_NEXT, "Media Next");
        m.insert(crate::VC_MEDIA_SELECT, "Media Select");
        m.insert(crate::VC_MEDIA_EJECT, "Media Eject");
        m.insert(crate::VC_VOLUME_MUTE, "Volume Mute");
        m.insert(crate::VC_VOLUME_UP, "Volume Up");
        m.insert(crate::VC_VOLUME_DOWN, "Volume Down");
        m.insert(crate::VC_APP_MAIL, "App Mail");
        m.insert(crate::VC_APP_CALCULATOR, "App Calculator");
        m.insert(crate::VC_APP_MUSIC, "App Music");
        m.insert(crate::VC_APP_PICTURES, "App Pictures");
        m.insert(crate::VC_BROWSER_SEARCH, "Browser Search");
        m.insert(crate::VC_BROWSER_HOME, "Browser Home");
        m.insert(crate::VC_BROWSER_BACK, "Browser Back");
        m.insert(crate::VC_BROWSER_FORWARD, "Browser Forward");
        m.insert(crate::VC_BROWSER_STOP, "Browser Stop");
        m.insert(crate::VC_BROWSER_REFRESH, "Browser Refresh");
        m.insert(crate::VC_BROWSER_FAVORITES, "Browser Favorites");
        m.insert(crate::VC_KATAKANA, "Katakana");
        m.insert(crate::VC_UNDERSCORE, "Underscore");
        m.insert(crate::VC_FURIGANA, "Furigana");
        m.insert(crate::VC_KANJI, "Kanji");
        m.insert(crate::VC_HIRAGANA, "Hiragana");
        m.insert(crate::VC_YEN, "Yen");
        m.insert(crate::VC_KP_COMMA, "Keypad Comma");
        m.insert(crate::VC_SUN_HELP, "Sun Help");
        m.insert(crate::VC_SUN_STOP, "Sun Stop");
        m.insert(crate::VC_SUN_PROPS, "Sun Props");
        m.insert(crate::VC_SUN_FRONT, "Sun Front");
        m.insert(crate::VC_SUN_OPEN, "Sun Open");
        m.insert(crate::VC_SUN_FIND, "Sun Find");
        m.insert(crate::VC_SUN_AGAIN, "Sun Again");
        m.insert(crate::VC_SUN_UNDO, "Sun Undo");
        m.insert(crate::VC_SUN_COPY, "Sun Copy");
        m.insert(crate::VC_SUN_INSERT, "Sun Insert");
        m.insert(crate::VC_SUN_CUT, "Sun Cut");
        m.insert(crate::VC_UNDEFINED, "Undefined");
        m.insert(crate::CHAR_UNDEFINED, "Undefined");

        m
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardEventType {
    Pressed,
    Released,
    Typed,
}

#[derive(Debug, Clone)]
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