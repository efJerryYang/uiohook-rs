use crate::{bindings, UiohookEvent};
use crate::error::UiohookError;
use crate::Uiohook;
use std::convert::TryFrom;

/// Represents the type of keyboard event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardEventType {
    /// A key was pressed down.
    Pressed,
    /// A key was released.
    Released,
    /// A character was typed (usually follows a press and release).
    Typed,
}

/// Represents a keyboard event.
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    /// The type of the keyboard event.
    pub event_type: KeyboardEventType,
    /// The key code of the event.
    pub key_code: KeyCode,
    /// The raw key code as received from the system.
    pub raw_code: u16,
    /// The character associated with the key, if applicable.
    pub key_char: Option<char>,
}

impl From<&bindings::keyboard_event_data> for KeyboardEvent {
    fn from(event: &bindings::keyboard_event_data) -> Self {
        KeyboardEvent {
            event_type: KeyboardEventType::Pressed, // This will be set correctly by the caller
            key_code: KeyCode::try_from(event.keycode as u32).unwrap_or(KeyCode::Undefined),
            raw_code: event.rawcode,
            key_char: char::from_u32(event.keychar as u32),
        }
    }
}

/// Represents a key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Function keys
    Escape,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
    
    // Alphanumeric keys
    Backquote,
    Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9, Num0,
    Minus, Equals, Backspace,
    Tab, CapsLock,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    OpenBracket, CloseBracket, Backslash,
    Semicolon, Quote, Enter,
    Comma, Period, Slash,
    Space,

    // Navigation keys
    PrintScreen, ScrollLock, Pause, LesserGreater,
    Insert, Delete, Home, End, PageUp, PageDown,
    Up, Left, Clear, Right, Down,

    // Numeric keypad
    NumLock,
    KpDivide, KpMultiply, KpSubtract, KpEquals, KpAdd, KpEnter, KpSeparator,
    Kp1, Kp2, Kp3, Kp4, Kp5, Kp6, Kp7, Kp8, Kp9, Kp0,
    KpEnd, KpDown, KpPageDown, KpLeft, KpClear, KpRight, KpHome, KpUp, KpPageUp,
    KpInsert, KpDelete,

    // Modifier keys
    ShiftL, ShiftR, ControlL, ControlR, AltL, AltR, MetaL, MetaR,
    
    // Additional keys
    ContextMenu, Power, Sleep, Wake,

    // Media keys
    MediaPlay, MediaStop, MediaPrevious, MediaNext, MediaSelect, MediaEject,
    VolumeMute, VolumeUp, VolumeDown,

    // Application keys
    AppMail, AppCalculator, AppMusic, AppPictures,

    // Browser keys
    BrowserSearch, BrowserHome, BrowserBack, BrowserForward, BrowserStop, BrowserRefresh, BrowserFavorites,

    // Japanese keys
    Katakana, Underscore, Furigana, Kanji, Hiragana, Yen, KpComma,

    // Sun keys
    SunHelp, SunStop, SunProps, SunFront, SunOpen, SunFind, SunAgain, SunUndo, SunCopy, SunInsert, SunCut,
    
    // Undefined key
    Undefined, CharUndefined,
}

impl TryFrom<u32> for KeyCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            bindings::VC_ESCAPE => Ok(KeyCode::Escape),
            bindings::VC_F1 => Ok(KeyCode::F1),
            bindings::VC_F2 => Ok(KeyCode::F2),
            bindings::VC_F3 => Ok(KeyCode::F3),
            bindings::VC_F4 => Ok(KeyCode::F4),
            bindings::VC_F5 => Ok(KeyCode::F5),
            bindings::VC_F6 => Ok(KeyCode::F6),
            bindings::VC_F7 => Ok(KeyCode::F7),
            bindings::VC_F8 => Ok(KeyCode::F8),
            bindings::VC_F9 => Ok(KeyCode::F9),
            bindings::VC_F10 => Ok(KeyCode::F10),
            bindings::VC_F11 => Ok(KeyCode::F11),
            bindings::VC_F12 => Ok(KeyCode::F12),
            bindings::VC_F13 => Ok(KeyCode::F13),
            bindings::VC_F14 => Ok(KeyCode::F14),
            bindings::VC_F15 => Ok(KeyCode::F15),
            bindings::VC_F16 => Ok(KeyCode::F16),
            bindings::VC_F17 => Ok(KeyCode::F17),
            bindings::VC_F18 => Ok(KeyCode::F18),
            bindings::VC_F19 => Ok(KeyCode::F19),
            bindings::VC_F20 => Ok(KeyCode::F20),
            bindings::VC_F21 => Ok(KeyCode::F21),
            bindings::VC_F22 => Ok(KeyCode::F22),
            bindings::VC_F23 => Ok(KeyCode::F23),
            bindings::VC_F24 => Ok(KeyCode::F24),
            bindings::VC_BACKQUOTE => Ok(KeyCode::Backquote),
            bindings::VC_1 => Ok(KeyCode::Num1),
            bindings::VC_2 => Ok(KeyCode::Num2),
            bindings::VC_3 => Ok(KeyCode::Num3),
            bindings::VC_4 => Ok(KeyCode::Num4),
            bindings::VC_5 => Ok(KeyCode::Num5),
            bindings::VC_6 => Ok(KeyCode::Num6),
            bindings::VC_7 => Ok(KeyCode::Num7),
            bindings::VC_8 => Ok(KeyCode::Num8),
            bindings::VC_9 => Ok(KeyCode::Num9),
            bindings::VC_0 => Ok(KeyCode::Num0),
            bindings::VC_MINUS => Ok(KeyCode::Minus),
            bindings::VC_EQUALS => Ok(KeyCode::Equals),
            bindings::VC_BACKSPACE => Ok(KeyCode::Backspace),
            bindings::VC_TAB => Ok(KeyCode::Tab),
            bindings::VC_CAPS_LOCK => Ok(KeyCode::CapsLock),
            bindings::VC_A => Ok(KeyCode::A),
            bindings::VC_B => Ok(KeyCode::B),
            bindings::VC_C => Ok(KeyCode::C),
            bindings::VC_D => Ok(KeyCode::D),
            bindings::VC_E => Ok(KeyCode::E),
            bindings::VC_F => Ok(KeyCode::F),
            bindings::VC_G => Ok(KeyCode::G),
            bindings::VC_H => Ok(KeyCode::H),
            bindings::VC_I => Ok(KeyCode::I),
            bindings::VC_J => Ok(KeyCode::J),
            bindings::VC_K => Ok(KeyCode::K),
            bindings::VC_L => Ok(KeyCode::L),
            bindings::VC_M => Ok(KeyCode::M),
            bindings::VC_N => Ok(KeyCode::N),
            bindings::VC_O => Ok(KeyCode::O),
            bindings::VC_P => Ok(KeyCode::P),
            bindings::VC_Q => Ok(KeyCode::Q),
            bindings::VC_R => Ok(KeyCode::R),
            bindings::VC_S => Ok(KeyCode::S),
            bindings::VC_T => Ok(KeyCode::T),
            bindings::VC_U => Ok(KeyCode::U),
            bindings::VC_V => Ok(KeyCode::V),
            bindings::VC_W => Ok(KeyCode::W),
            bindings::VC_X => Ok(KeyCode::X),
            bindings::VC_Y => Ok(KeyCode::Y),
            bindings::VC_Z => Ok(KeyCode::Z),
            bindings::VC_OPEN_BRACKET => Ok(KeyCode::OpenBracket),
            bindings::VC_CLOSE_BRACKET => Ok(KeyCode::CloseBracket),
            bindings::VC_BACK_SLASH => Ok(KeyCode::Backslash),
            bindings::VC_SEMICOLON => Ok(KeyCode::Semicolon),
            bindings::VC_QUOTE => Ok(KeyCode::Quote),
            bindings::VC_ENTER => Ok(KeyCode::Enter),
            bindings::VC_COMMA => Ok(KeyCode::Comma),
            bindings::VC_PERIOD => Ok(KeyCode::Period),
            bindings::VC_SLASH => Ok(KeyCode::Slash),
            bindings::VC_SPACE => Ok(KeyCode::Space),
            bindings::VC_PRINTSCREEN => Ok(KeyCode::PrintScreen),
            bindings::VC_SCROLL_LOCK => Ok(KeyCode::ScrollLock),
            bindings::VC_PAUSE => Ok(KeyCode::Pause),
            bindings::VC_LESSER_GREATER => Ok(KeyCode::LesserGreater),
            bindings::VC_INSERT => Ok(KeyCode::Insert),
            bindings::VC_DELETE => Ok(KeyCode::Delete),
            bindings::VC_HOME => Ok(KeyCode::Home),
            bindings::VC_END => Ok(KeyCode::End),
            bindings::VC_PAGE_UP => Ok(KeyCode::PageUp),
            bindings::VC_PAGE_DOWN => Ok(KeyCode::PageDown),
            bindings::VC_UP => Ok(KeyCode::Up),
            bindings::VC_LEFT => Ok(KeyCode::Left),
            bindings::VC_CLEAR => Ok(KeyCode::Clear),
            bindings::VC_RIGHT => Ok(KeyCode::Right),
            bindings::VC_DOWN => Ok(KeyCode::Down),
            bindings::VC_NUM_LOCK => Ok(KeyCode::NumLock),
            bindings::VC_KP_DIVIDE => Ok(KeyCode::KpDivide),
            bindings::VC_KP_MULTIPLY => Ok(KeyCode::KpMultiply),
            bindings::VC_KP_SUBTRACT => Ok(KeyCode::KpSubtract),
            bindings::VC_KP_EQUALS => Ok(KeyCode::KpEquals),
            bindings::VC_KP_ADD => Ok(KeyCode::KpAdd),
            bindings::VC_KP_ENTER => Ok(KeyCode::KpEnter),
            bindings::VC_KP_SEPARATOR => Ok(KeyCode::KpSeparator),
            bindings::VC_KP_1 => Ok(KeyCode::Kp1),
            bindings::VC_KP_2 => Ok(KeyCode::Kp2),
            bindings::VC_KP_3 => Ok(KeyCode::Kp3),
            bindings::VC_KP_4 => Ok(KeyCode::Kp4),
            bindings::VC_KP_5 => Ok(KeyCode::Kp5),
            bindings::VC_KP_6 => Ok(KeyCode::Kp6),
            bindings::VC_KP_7 => Ok(KeyCode::Kp7),
            bindings::VC_KP_8 => Ok(KeyCode::Kp8),
            bindings::VC_KP_9 => Ok(KeyCode::Kp9),
            bindings::VC_KP_0 => Ok(KeyCode::Kp0),
            bindings::VC_KP_END => Ok(KeyCode::KpEnd),
            bindings::VC_KP_DOWN => Ok(KeyCode::KpDown),
            bindings::VC_KP_PAGE_DOWN => Ok(KeyCode::KpPageDown),
            bindings::VC_KP_LEFT => Ok(KeyCode::KpLeft),
            bindings::VC_KP_CLEAR => Ok(KeyCode::KpClear),
            bindings::VC_KP_RIGHT => Ok(KeyCode::KpRight),
            bindings::VC_KP_HOME => Ok(KeyCode::KpHome),
            bindings::VC_KP_UP => Ok(KeyCode::KpUp),
            bindings::VC_KP_PAGE_UP => Ok(KeyCode::KpPageUp),
            bindings::VC_KP_INSERT => Ok(KeyCode::KpInsert),
            bindings::VC_KP_DELETE => Ok(KeyCode::KpDelete),
            bindings::VC_SHIFT_L => Ok(KeyCode::ShiftL),
            bindings::VC_SHIFT_R => Ok(KeyCode::ShiftR),
            bindings::VC_CONTROL_L => Ok(KeyCode::ControlL),
            bindings::VC_CONTROL_R => Ok(KeyCode::ControlR),
            bindings::VC_ALT_L => Ok(KeyCode::AltL),
            bindings::VC_ALT_R => Ok(KeyCode::AltR),
            bindings::VC_META_L => Ok(KeyCode::MetaL),
            bindings::VC_META_R => Ok(KeyCode::MetaR),
            bindings::VC_CONTEXT_MENU => Ok(KeyCode::ContextMenu),
            bindings::VC_POWER => Ok(KeyCode::Power),
            bindings::VC_SLEEP => Ok(KeyCode::Sleep),
            bindings::VC_WAKE => Ok(KeyCode::Wake),
            bindings::VC_MEDIA_PLAY => Ok(KeyCode::MediaPlay),
            bindings::VC_MEDIA_STOP => Ok(KeyCode::MediaStop),
            bindings::VC_MEDIA_PREVIOUS => Ok(KeyCode::MediaPrevious),
            bindings::VC_MEDIA_NEXT => Ok(KeyCode::MediaNext),
            bindings::VC_MEDIA_SELECT => Ok(KeyCode::MediaSelect),
            bindings::VC_MEDIA_EJECT => Ok(KeyCode::MediaEject),
            bindings::VC_VOLUME_MUTE => Ok(KeyCode::VolumeMute),
            bindings::VC_VOLUME_UP => Ok(KeyCode::VolumeUp),
            bindings::VC_VOLUME_DOWN => Ok(KeyCode::VolumeDown),
            bindings::VC_APP_MAIL => Ok(KeyCode::AppMail),
            bindings::VC_APP_CALCULATOR => Ok(KeyCode::AppCalculator),
            bindings::VC_APP_MUSIC => Ok(KeyCode::AppMusic),
            bindings::VC_APP_PICTURES => Ok(KeyCode::AppPictures),
            bindings::VC_BROWSER_SEARCH => Ok(KeyCode::BrowserSearch),
            bindings::VC_BROWSER_HOME => Ok(KeyCode::BrowserHome),
            bindings::VC_BROWSER_BACK => Ok(KeyCode::BrowserBack),
            bindings::VC_BROWSER_FORWARD => Ok(KeyCode::BrowserForward),
            bindings::VC_BROWSER_STOP => Ok(KeyCode::BrowserStop),
            bindings::VC_BROWSER_REFRESH => Ok(KeyCode::BrowserRefresh),
            bindings::VC_BROWSER_FAVORITES => Ok(KeyCode::BrowserFavorites),
            bindings::VC_KATAKANA => Ok(KeyCode::Katakana),
            bindings::VC_UNDERSCORE => Ok(KeyCode::Underscore),
            bindings::VC_FURIGANA => Ok(KeyCode::Furigana),
            bindings::VC_KANJI => Ok(KeyCode::Kanji),
            bindings::VC_HIRAGANA => Ok(KeyCode::Hiragana),
            bindings::VC_YEN => Ok(KeyCode::Yen),
            bindings::VC_KP_COMMA => Ok(KeyCode::KpComma),
            bindings::VC_SUN_HELP => Ok(KeyCode::SunHelp),
            bindings::VC_SUN_STOP => Ok(KeyCode::SunStop),
            bindings::VC_SUN_PROPS => Ok(KeyCode::SunProps),
            bindings::VC_SUN_FRONT => Ok(KeyCode::SunFront),
            bindings::VC_SUN_OPEN => Ok(KeyCode::SunOpen),
            bindings::VC_SUN_FIND => Ok(KeyCode::SunFind),
            bindings::VC_SUN_AGAIN => Ok(KeyCode::SunAgain),
            bindings::VC_SUN_UNDO => Ok(KeyCode::SunUndo),
            bindings::VC_SUN_COPY => Ok(KeyCode::SunCopy),
            bindings::VC_SUN_INSERT => Ok(KeyCode::SunInsert),
            bindings::VC_SUN_CUT => Ok(KeyCode::SunCut),
            bindings::VC_UNDEFINED => Ok(KeyCode::Undefined),
            bindings::CHAR_UNDEFINED => Ok(KeyCode::CharUndefined),
            _ => Err(()),
        }
    }
}

impl From<KeyCode> for u32 {
    fn from(key_code: KeyCode) -> Self {
        match key_code {
            KeyCode::Escape => bindings::VC_ESCAPE,
            KeyCode::F1 => bindings::VC_F1,
            KeyCode::F2 => bindings::VC_F2,
            KeyCode::F3 => bindings::VC_F3,
            KeyCode::F4 => bindings::VC_F4,
            KeyCode::F5 => bindings::VC_F5,
            KeyCode::F6 => bindings::VC_F6,
            KeyCode::F7 => bindings::VC_F7,
            KeyCode::F8 => bindings::VC_F8,
            KeyCode::F9 => bindings::VC_F9,
            KeyCode::F10 => bindings::VC_F10,
            KeyCode::F11 => bindings::VC_F11,
            KeyCode::F12 => bindings::VC_F12,
            KeyCode::F13 => bindings::VC_F13,
            KeyCode::F14 => bindings::VC_F14,
            KeyCode::F15 => bindings::VC_F15,
            KeyCode::F16 => bindings::VC_F16,
            KeyCode::F17 => bindings::VC_F17,
            KeyCode::F18 => bindings::VC_F18,
            KeyCode::F19 => bindings::VC_F19,
            KeyCode::F20 => bindings::VC_F20,
            KeyCode::F21 => bindings::VC_F21,
            KeyCode::F22 => bindings::VC_F22,
            KeyCode::F23 => bindings::VC_F23,
            KeyCode::F24 => bindings::VC_F24,
            KeyCode::Backquote => bindings::VC_BACKQUOTE,
            KeyCode::Num1 => bindings::VC_1,
            KeyCode::Num2 => bindings::VC_2,
            KeyCode::Num3 => bindings::VC_3,
            KeyCode::Num4 => bindings::VC_4,
            KeyCode::Num5 => bindings::VC_5,
            KeyCode::Num6 => bindings::VC_6,
            KeyCode::Num7 => bindings::VC_7,
            KeyCode::Num8 => bindings::VC_8,
            KeyCode::Num9 => bindings::VC_9,
            KeyCode::Num0 => bindings::VC_0,
            KeyCode::Minus => bindings::VC_MINUS,
            KeyCode::Equals => bindings::VC_EQUALS,
            KeyCode::Backspace => bindings::VC_BACKSPACE,
            KeyCode::Tab => bindings::VC_TAB,
            KeyCode::CapsLock => bindings::VC_CAPS_LOCK,
            KeyCode::A => bindings::VC_A,
            KeyCode::B => bindings::VC_B,
            KeyCode::C => bindings::VC_C,
            KeyCode::D => bindings::VC_D,
            KeyCode::E => bindings::VC_E,
            KeyCode::F => bindings::VC_F,
            KeyCode::G => bindings::VC_G,
            KeyCode::H => bindings::VC_H,
            KeyCode::I => bindings::VC_I,
            KeyCode::J => bindings::VC_J,
            KeyCode::K => bindings::VC_K,
            KeyCode::L => bindings::VC_L,
            KeyCode::M => bindings::VC_M,
            KeyCode::N => bindings::VC_N,
            KeyCode::O => bindings::VC_O,
            KeyCode::P => bindings::VC_P,
            KeyCode::Q => bindings::VC_Q,
            KeyCode::R => bindings::VC_R,
            KeyCode::S => bindings::VC_S,
            KeyCode::T => bindings::VC_T,
            KeyCode::U => bindings::VC_U,
            KeyCode::V => bindings::VC_V,
            KeyCode::W => bindings::VC_W,
            KeyCode::X => bindings::VC_X,
            KeyCode::Y => bindings::VC_Y,
            KeyCode::Z => bindings::VC_Z,
            KeyCode::OpenBracket => bindings::VC_OPEN_BRACKET,
            KeyCode::CloseBracket => bindings::VC_CLOSE_BRACKET,
            KeyCode::Backslash => bindings::VC_BACK_SLASH,
            KeyCode::Semicolon => bindings::VC_SEMICOLON,
            KeyCode::Quote => bindings::VC_QUOTE,
            KeyCode::Enter => bindings::VC_ENTER,
            KeyCode::Comma => bindings::VC_COMMA,
            KeyCode::Period => bindings::VC_PERIOD,
            KeyCode::Slash => bindings::VC_SLASH,
            KeyCode::Space => bindings::VC_SPACE,
            KeyCode::PrintScreen => bindings::VC_PRINTSCREEN,
            KeyCode::ScrollLock => bindings::VC_SCROLL_LOCK,
            KeyCode::Pause => bindings::VC_PAUSE,
            KeyCode::LesserGreater => bindings::VC_LESSER_GREATER,
            KeyCode::Insert => bindings::VC_INSERT,
            KeyCode::Delete => bindings::VC_DELETE,
            KeyCode::Home => bindings::VC_HOME,
            KeyCode::End => bindings::VC_END,
            KeyCode::PageUp => bindings::VC_PAGE_UP,
            KeyCode::PageDown => bindings::VC_PAGE_DOWN,
            KeyCode::Up => bindings::VC_UP,
            KeyCode::Left => bindings::VC_LEFT,
            KeyCode::Clear => bindings::VC_CLEAR,
            KeyCode::Right => bindings::VC_RIGHT,
            KeyCode::Down => bindings::VC_DOWN,
            KeyCode::NumLock => bindings::VC_NUM_LOCK,
            KeyCode::KpDivide => bindings::VC_KP_DIVIDE,
            KeyCode::KpMultiply => bindings::VC_KP_MULTIPLY,
            KeyCode::KpSubtract => bindings::VC_KP_SUBTRACT,
            KeyCode::KpEquals => bindings::VC_KP_EQUALS,
            KeyCode::KpAdd => bindings::VC_KP_ADD,
            KeyCode::KpEnter => bindings::VC_KP_ENTER,
            KeyCode::KpSeparator => bindings::VC_KP_SEPARATOR,
            KeyCode::Kp1 => bindings::VC_KP_1,
            KeyCode::Kp2 => bindings::VC_KP_2,
            KeyCode::Kp3 => bindings::VC_KP_3,
            KeyCode::Kp4 => bindings::VC_KP_4,
            KeyCode::Kp5 => bindings::VC_KP_5,
            KeyCode::Kp6 => bindings::VC_KP_6,
            KeyCode::Kp7 => bindings::VC_KP_7,
            KeyCode::Kp8 => bindings::VC_KP_8,
            KeyCode::Kp9 => bindings::VC_KP_9,
            KeyCode::Kp0 => bindings::VC_KP_0,
            KeyCode::KpEnd => bindings::VC_KP_END,
            KeyCode::KpDown => bindings::VC_KP_DOWN,
            KeyCode::KpPageDown => bindings::VC_KP_PAGE_DOWN,
            KeyCode::KpLeft => bindings::VC_KP_LEFT,
            KeyCode::KpClear => bindings::VC_KP_CLEAR,
            KeyCode::KpRight => bindings::VC_KP_RIGHT,
            KeyCode::KpHome => bindings::VC_KP_HOME,
            KeyCode::KpUp => bindings::VC_KP_UP,
            KeyCode::KpPageUp => bindings::VC_KP_PAGE_UP,
            KeyCode::KpInsert => bindings::VC_KP_INSERT,
            KeyCode::KpDelete => bindings::VC_KP_DELETE,
            KeyCode::ShiftL => bindings::VC_SHIFT_L,
            KeyCode::ShiftR => bindings::VC_SHIFT_R,
            KeyCode::ControlL => bindings::VC_CONTROL_L,
            KeyCode::ControlR => bindings::VC_CONTROL_R,
            KeyCode::AltL => bindings::VC_ALT_L,
            KeyCode::AltR => bindings::VC_ALT_R,
            KeyCode::MetaL => bindings::VC_META_L,
            KeyCode::MetaR => bindings::VC_META_R,
            KeyCode::ContextMenu => bindings::VC_CONTEXT_MENU,
            KeyCode::Power => bindings::VC_POWER,
            KeyCode::Sleep => bindings::VC_SLEEP,
            KeyCode::Wake => bindings::VC_WAKE,
            KeyCode::MediaPlay => bindings::VC_MEDIA_PLAY,
            KeyCode::MediaStop => bindings::VC_MEDIA_STOP,
            KeyCode::MediaPrevious => bindings::VC_MEDIA_PREVIOUS,
            KeyCode::MediaNext => bindings::VC_MEDIA_NEXT,
            KeyCode::MediaSelect => bindings::VC_MEDIA_SELECT,
            KeyCode::MediaEject => bindings::VC_MEDIA_EJECT,
            KeyCode::VolumeMute => bindings::VC_VOLUME_MUTE,
            KeyCode::VolumeUp => bindings::VC_VOLUME_UP,
            KeyCode::VolumeDown => bindings::VC_VOLUME_DOWN,
            KeyCode::AppMail => bindings::VC_APP_MAIL,
            KeyCode::AppCalculator => bindings::VC_APP_CALCULATOR,
            KeyCode::AppMusic => bindings::VC_APP_MUSIC,
            KeyCode::AppPictures => bindings::VC_APP_PICTURES,
            KeyCode::BrowserSearch => bindings::VC_BROWSER_SEARCH,
            KeyCode::BrowserHome => bindings::VC_BROWSER_HOME,
            KeyCode::BrowserBack => bindings::VC_BROWSER_BACK,
            KeyCode::BrowserForward => bindings::VC_BROWSER_FORWARD,
            KeyCode::BrowserStop => bindings::VC_BROWSER_STOP,
            KeyCode::BrowserRefresh => bindings::VC_BROWSER_REFRESH,
            KeyCode::BrowserFavorites => bindings::VC_BROWSER_FAVORITES,
            KeyCode::Katakana => bindings::VC_KATAKANA,
            KeyCode::Underscore => bindings::VC_UNDERSCORE,
            KeyCode::Furigana => bindings::VC_FURIGANA,
            KeyCode::Kanji => bindings::VC_KANJI,
            KeyCode::Hiragana => bindings::VC_HIRAGANA,
            KeyCode::Yen => bindings::VC_YEN,
            KeyCode::KpComma => bindings::VC_KP_COMMA,
            KeyCode::SunHelp => bindings::VC_SUN_HELP,
            KeyCode::SunStop => bindings::VC_SUN_STOP,
            KeyCode::SunProps => bindings::VC_SUN_PROPS,
            KeyCode::SunFront => bindings::VC_SUN_FRONT,
            KeyCode::SunOpen => bindings::VC_SUN_OPEN,
            KeyCode::SunFind => bindings::VC_SUN_FIND,
            KeyCode::SunAgain => bindings::VC_SUN_AGAIN,
            KeyCode::SunUndo => bindings::VC_SUN_UNDO,
            KeyCode::SunCopy => bindings::VC_SUN_COPY,
            KeyCode::SunInsert => bindings::VC_SUN_INSERT,
            KeyCode::SunCut => bindings::VC_SUN_CUT,
            KeyCode::Undefined => bindings::VC_UNDEFINED,
            KeyCode::CharUndefined => bindings::CHAR_UNDEFINED,
        }
    }
}

/// Simulates a key tap (press and release) for the given key code.
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `key` - The key code to tap.
/// * `modifiers` - An optional slice of modifier key codes to be held during the tap.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, keyboard::{key_tap, KeyCode}};
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
/// key_tap(&hook, KeyCode::A, &[KeyCode::ShiftL]).expect("Failed to tap key");
/// ```
pub fn key_tap(uiohook: &Uiohook, key: KeyCode, modifiers: &[KeyCode]) -> Result<(), UiohookError> {
    // 1. Create keyboard events for pressing modifiers
    for &modifier in modifiers {
        let press_event = create_keyboard_event(KeyboardEventType::Pressed, modifier);
        uiohook.post_event(&UiohookEvent::Keyboard(press_event))?;
    }

    // 2. Create a keyboard event for pressing the key
    let key_press_event = create_keyboard_event(KeyboardEventType::Pressed, key);
    uiohook.post_event(&UiohookEvent::Keyboard(key_press_event))?;

    // 3. Create a keyboard event for releasing the key
    let key_release_event = create_keyboard_event(KeyboardEventType::Released, key);
    uiohook.post_event(&UiohookEvent::Keyboard(key_release_event))?;

    // 4. Create keyboard events for releasing modifiers
    for &modifier in modifiers.iter().rev() {
        let release_event = create_keyboard_event(KeyboardEventType::Released, modifier);
        uiohook.post_event(&UiohookEvent::Keyboard(release_event))?;
    }

    Ok(())
}

/// Simulates a key press or release for the given key code.
///
/// # Arguments
///
/// * `uiohook` - A reference to the Uiohook instance.
/// * `key` - The key code to toggle.
/// * `down` - If true, simulates a key press. If false, simulates a key release.
///
/// # Returns
///
/// A `Result` indicating success or an error if the operation failed.
///
/// # Examples
///
/// ```no_run
/// use uiohook_rs::{Uiohook, EventHandler, UiohookEvent, keyboard::{key_toggle, KeyCode}};
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
/// key_toggle(&hook, KeyCode::ShiftL, true).expect("Failed to press key");
/// key_toggle(&hook, KeyCode::ShiftL, false).expect("Failed to release key");
/// ```

pub fn key_toggle(uiohook: &Uiohook, key: KeyCode, down: bool) -> Result<(), UiohookError> {
    // 1. Create a keyboard event for pressing or releasing the key based on the 'down' parameter
    let event_type = if down { KeyboardEventType::Pressed } else { KeyboardEventType::Released };
    let event = create_keyboard_event(event_type, key);

    // 2. Use Uiohook::post_event to send this event
    uiohook.post_event(&UiohookEvent::Keyboard(event))?;

    Ok(())
}


// Helper function to create a KeyboardEvent
fn create_keyboard_event(event_type: KeyboardEventType, key: KeyCode) -> KeyboardEvent {
    KeyboardEvent {
        event_type,
        key_code: key,
        raw_code: u32::from(key) as u16, // Cast to u16 as raw_code is u16
        key_char: None, // We don't have character information for simulated events
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_code_conversion() {
        assert_eq!(KeyCode::try_from(bindings::VC_ESCAPE), Ok(KeyCode::Escape));
        assert_eq!(KeyCode::try_from(bindings::VC_F1), Ok(KeyCode::F1));
        assert_eq!(KeyCode::try_from(0xFFFFFFFF), Err(()));

        assert_eq!(u32::from(KeyCode::Escape), bindings::VC_ESCAPE);
        assert_eq!(u32::from(KeyCode::F1), bindings::VC_F1);
    }

    #[test]
    fn test_keyboard_event_from_bindings() {
        let binding_event = bindings::keyboard_event_data {
            keycode: bindings::VC_A as u16,
            rawcode: 65,
            keychar: 'A' as u16,
        };

        let event = KeyboardEvent::from(&binding_event);
        assert_eq!(event.key_code, KeyCode::A);
        assert_eq!(event.raw_code, 65);
        assert_eq!(event.key_char, Some('A'));
    }

    // Add more tests as needed
}