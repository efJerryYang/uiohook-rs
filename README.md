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

1. Clone the repo:

   ```sh
   git clone https://github.com/efJerryYang/uiohook-rs.git
   cd uiohook-rs
   ```

2. Run the demo:

   ```sh
   cargo run --example demo
   ```

3. Press Ctrl-C to exit.

## License

GNU General Public License v3.0, see [LICENSE](LICENSE).
