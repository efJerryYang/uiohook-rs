use uiohook_rs::hook::wheel::WheelEvent;
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoWheelHandler;

impl EventHandler for DemoWheelHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        if let UiohookEvent::Wheel(wheel_event) = event {
            self.handle_wheel_event(wheel_event);
        }
    }
}

impl DemoWheelHandler {
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

    let event_handler = DemoWheelHandler;

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
    }

    println!("Exiting...");
}
