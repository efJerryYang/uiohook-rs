# uiohook-rs

[![Crates.io](https://img.shields.io/crates/v/uiohook-rs.svg)](https://crates.io/crates/uiohook-rs)

`uiohook-rs` is a Rust wrapper for the [libuiohook](https://github.com/kwhat/libuiohook), providing cross-platform keyboard and mouse hooking capabilities.

## Features

- Cross-platform support (Linux, macOS, Windows)
- Low-level keyboard and mouse event handling
- Easy-to-use Rust API

**Note**: All examples have now been tested on macOS, Windows and Linux.  
On macOS the examples have been updated to use the `CoreFoundation` run loop (`CFRunLoop`) for proper event dispatch and exit.

## Usage

Here's a basic example of how to use `uiohook-rs`:

```rust
use uiohook_rs::{EventHandler, Uiohook, UiohookEvent};

struct MyEventHandler;

impl EventHandler for MyEventHandler {
    fn handle_event(&self, event: &UiohookEvent) {
        println!("Event: {:?}", event);
    }
}

fn main() {
    let event_handler = MyEventHandler;
    let uiohook = Uiohook::new(event_handler);

    if let Err(e) = uiohook.run() {
        eprintln!("Error: {}", e);
    }
    
    // Stop the hook
    if let Err(e) = uiohook.stop() {
        eprintln!("Error: {}", e);
    }
}
```

## Running the Demo

There are several example programs available under the `examples/` directory, including demos for general event handling (`demo.rs`), pretty-printed output (`pretty_demo.rs`), and specific handlers for keyboard, mouse, and wheel events. Except for the `pretty_demo.rs`, all other examples use the minimal code to demonstrate the hook functionality.

To run the demo:

1. Clone the repo:

   ```sh
   git clone https://github.com/efJerryYang/uiohook-rs.git
   cd uiohook-rs
   ```

2. Run the demo:

   ```sh
   cargo run --example demo
   ```

3. Press Ctrl-C to exit. See the output:

    ```txt
    Press Ctrl-C to exit
    MOVED    | Mouse             | X: 802   | Y: 644   | Button: 0    | Clicks: 0   
    PRESSED  | Caps Lock         | Code: 58    | Raw: 65509
    RELEASED | Caps Lock         | Code: 58    | Raw: 65509
    PRESSED  | Caps Lock         | Code: 58    | Raw: 65509
    RELEASED | Caps Lock         | Code: 58    | Raw: 65509
    PRESSED  | Left Shift        | Code: 42    | Raw: 65505
    PRESSED  | B                 | Code: 48    | Raw: 66   
    TYPED    | B                 | Code: 66    | Raw: 66   
    RELEASED | B                 | Code: 48    | Raw: 66   
    RELEASED | Left Shift        | Code: 42    | Raw: 65505
    PRESSED  | H                 | Code: 35    | Raw: 104  
    TYPED    | h                 | Code: 104   | Raw: 104  
    RELEASED | H                 | Code: 35    | Raw: 104  
    PRESSED  | Mouse             | X: 802   | Y: 644   | Button: 1    | Clicks: 1   
    RELEASED | Mouse             | X: 802   | Y: 644   | Button: 1    | Clicks: 1   
    CLICKED  | Mouse             | X: 802   | Y: 644   | Button: 1    | Clicks: 1   
    PRESSED  | Mouse             | X: 802   | Y: 644   | Button: 1    | Clicks: 1   
    DRAGGED  | Mouse             | X: 803   | Y: 644   | Button: 0    | Clicks: 1   
    ... (more DRAGGED events)
    DRAGGED  | Mouse             | X: 920   | Y: 630   | Button: 0    | Clicks: 0   
    DRAGGED  | Mouse             | X: 921   | Y: 630   | Button: 0    | Clicks: 0   
    RELEASED | Mouse             | X: 921   | Y: 630   | Button: 1    | Clicks: 0   
    PRESSED  | Left Control      | Code: 29    | Raw: 65507
    PRESSED  | C                 | Code: 46    | Raw: 99   
    TYPED    | c                 | Code: 99    | Raw: 99   
    Exiting...
    ```

## License

GNU General Public License v3.0, see [LICENSE](LICENSE).
