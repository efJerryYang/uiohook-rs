use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct DemoStopHandler {
    event_count: Arc<AtomicI32>,
}

impl EventHandler for DemoStopHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        println!("Event received: {:?}", event);
        self.event_count.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    println!("Starting uiohook. Will stop after 5 seconds.");
    println!("Move your mouse or press keys to generate events.");

    let event_count = Arc::new(AtomicI32::new(0));
    let event_handler = DemoStopHandler {
        event_count: event_count.clone(),
    };

    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Failed to run uiohook: {}", e);
        return;
    }

    // Run for 5 seconds
    thread::sleep(Duration::from_secs(5));

    // Stop uiohook
    println!("Stopping uiohook...");
    if let Err(e) = uiohook.stop() {
        eprintln!("Failed to stop uiohook: {}", e);
    }

    let total_events = event_count.load(Ordering::SeqCst);
    println!("Uiohook stopped. Total events captured: {}", total_events);
    println!("Exiting...");
}