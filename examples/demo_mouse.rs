use uiohook_rs::hook::mouse::{MouseEvent, MouseEventType};
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoMouseHandler;

impl EventHandler for DemoMouseHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        if let UiohookEvent::Mouse(mouse_event) = event {
            self.handle_mouse_event(mouse_event);
        }
    }
}

impl DemoMouseHandler {
    fn handle_mouse_event(&self, mouse_event: &MouseEvent) {
        let event_type = match mouse_event.event_type {
            MouseEventType::Moved => "MOVED",
            MouseEventType::Pressed => "PRESSED",
            MouseEventType::Released => "RELEASED",
            MouseEventType::Clicked => "CLICKED",
            MouseEventType::Dragged => "DRAGGED",
        };

        let details = format!(
            "Button: {:<8} | Clicks: {:<4}",
            format!("{:?}", mouse_event.button),
            mouse_event.clicks
        );

        println!(
            "{:<8} | X: {:<5} | Y: {:<5} | {}",
            event_type,
            mouse_event.x,
            mouse_event.y,
            details
        );
    }
}

fn main() {
    println!("Running... Press Ctrl-C to exit");

    let event_handler = DemoMouseHandler;

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
    }

    println!("Exiting...");
}
