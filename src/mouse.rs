use crate::{event_type, UiohookEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Moved,
    Pressed,
    Released,
    Clicked,
    Dragged,
    Wheel,
}

#[derive(Debug, Clone)]
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