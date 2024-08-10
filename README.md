# uiohook-rs

[![Crates.io](https://img.shields.io/crates/v/uiohook-rs.svg)](https://crates.io/crates/uiohook-rs)

`uiohook-rs` is a Rust wrapper for the [libuiohook](https://github.com/kwhat/libuiohook), providing cross-platform keyboard and mouse hooking capabilities.

## Features

- Cross-platform support (Linux, macOS, Windows)
- Low-level keyboard and mouse event handling
- Easy-to-use Rust API

**IMPORTANT**: This crate has not been tested on `MacOS` and `Windows` yet, please report any issues you encounter (likely to be compilation issues related to dependencies).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
uiohook-rs = "0.1.0"
```

## Usage

Here's a basic example of how to use `uiohook-rs`:

```rust
use uiohook_rs::{run, set_dispatch_proc, stop, UiohookEvent};

fn main() {
    set_dispatch_proc(|event: &UiohookEvent| {
        println!("Event: {:?}", event);
    });

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }

    // To stop the hook:
    // if let Err(e) = stop() {
    //     eprintln!("Error stopping: {}", e);
    // }
}
```

## Running the Demo

There is a simple demo program [examples/demo.rs](examples/demo.rs) with pretty-printed output of keyboard and mouse events.

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

    ```sh
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
