use crate::bindings;

/// Constants for wheel scroll directions
pub const WHEEL_VERTICAL_DIRECTION: u8 = bindings::WHEEL_VERTICAL_DIRECTION as u8;
/// Constant for horizontal wheel scroll direction
pub const WHEEL_HORIZONTAL_DIRECTION: u8 = bindings::WHEEL_HORIZONTAL_DIRECTION as u8;

/// Represents a mouse wheel event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WheelEvent {
    /// The number of clicks.
    pub clicks: u16,
    /// The x-coordinate of the mouse pointer.
    pub x: i16,
    /// The y-coordinate of the mouse pointer.
    pub y: i16,
    /// The type of mouse wheel event.
    pub type_: u8,
    /// The amount of scrolling.
    pub amount: u16,
    /// The number of rotation units the mouse wheel was scrolled.
    pub rotation: i16,
    /// The direction of the scroll (vertical or horizontal).
    pub direction: u8,
}

impl From<&bindings::mouse_wheel_event_data> for WheelEvent {
    fn from(event: &bindings::mouse_wheel_event_data) -> Self {
        WheelEvent {
            clicks: event.clicks,
            x: event.x,
            y: event.y,
            type_: event.type_,
            amount: event.amount,
            rotation: event.rotation,
            direction: event.direction,
        }
    }
}

impl WheelEvent {
    /// Creates a new `WheelEvent` instance.
    ///
    /// # Arguments
    ///
    /// * `clicks` - The number of clicks.
    /// * `x` - The x-coordinate of the mouse pointer.
    /// * `y` - The y-coordinate of the mouse pointer.
    /// * `type_` - The type of mouse wheel event.
    /// * `amount` - The amount of scrolling.
    /// * `rotation` - The number of rotation units the mouse wheel was scrolled.
    /// * `direction` - The direction of the scroll (vertical or horizontal).
    ///
    /// # Returns
    ///
    /// A new `WheelEvent` instance.
    pub fn new(clicks: u16, x: i16, y: i16, type_: u8, amount: u16, rotation: i16, direction: u8) -> Self {
        WheelEvent {
            clicks,
            x,
            y,
            type_,
            amount,
            rotation,
            direction,
        }
    }

    /// Checks if the wheel event is a vertical scroll.
    ///
    /// # Returns
    ///
    /// `true` if the event is a vertical scroll, `false` otherwise.
    pub fn is_vertical(&self) -> bool {
        self.direction == WHEEL_VERTICAL_DIRECTION
    }

    /// Checks if the wheel event is a horizontal scroll.
    ///
    /// # Returns
    ///
    /// `true` if the event is a horizontal scroll, `false` otherwise.
    pub fn is_horizontal(&self) -> bool {
        self.direction == WHEEL_HORIZONTAL_DIRECTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheel_event_from_bindings() {
        let raw_event = bindings::mouse_wheel_event_data {
            clicks: 1,
            x: 100,
            y: 200,
            type_: bindings::WHEEL_UNIT_SCROLL as u8,
            amount: 3,
            rotation: -120,
            direction: bindings::WHEEL_VERTICAL_DIRECTION as u8,
        };

        let wheel_event = WheelEvent::from(&raw_event);

        assert_eq!(wheel_event.clicks, 1);
        assert_eq!(wheel_event.x, 100);
        assert_eq!(wheel_event.y, 200);
        assert_eq!(wheel_event.type_, bindings::WHEEL_UNIT_SCROLL as u8);
        assert_eq!(wheel_event.amount, 3);
        assert_eq!(wheel_event.rotation, -120);
        assert_eq!(wheel_event.direction, WHEEL_VERTICAL_DIRECTION);
    }

    #[test]
    fn test_wheel_event_new() {
        let wheel_event = WheelEvent::new(1, 100, 200, bindings::WHEEL_UNIT_SCROLL as u8, 3, -120, WHEEL_VERTICAL_DIRECTION);

        assert_eq!(wheel_event.clicks, 1);
        assert_eq!(wheel_event.x, 100);
        assert_eq!(wheel_event.y, 200);
        assert_eq!(wheel_event.type_, bindings::WHEEL_UNIT_SCROLL as u8);
        assert_eq!(wheel_event.amount, 3);
        assert_eq!(wheel_event.rotation, -120);
        assert_eq!(wheel_event.direction, WHEEL_VERTICAL_DIRECTION);
    }

    #[test]
    fn test_is_vertical() {
        let vertical_event = WheelEvent::new(1, 100, 200, bindings::WHEEL_UNIT_SCROLL as u8, 3, -120, WHEEL_VERTICAL_DIRECTION);
        let horizontal_event = WheelEvent::new(1, 100, 200, bindings::WHEEL_UNIT_SCROLL as u8, 3, -120, WHEEL_HORIZONTAL_DIRECTION);

        assert!(vertical_event.is_vertical());
        assert!(!horizontal_event.is_vertical());
    }

    #[test]
    fn test_is_horizontal() {
        let vertical_event = WheelEvent::new(1, 100, 200, bindings::WHEEL_UNIT_SCROLL as u8, 3, -120, WHEEL_VERTICAL_DIRECTION);
        let horizontal_event = WheelEvent::new(1, 100, 200, bindings::WHEEL_UNIT_SCROLL as u8, 3, -120, WHEEL_HORIZONTAL_DIRECTION);

        assert!(!vertical_event.is_horizontal());
        assert!(horizontal_event.is_horizontal());
    }
}