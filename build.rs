use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libuiohook_dir = root.join("libuiohook");

    println!("cargo:rustc-link-search={}", libuiohook_dir.display());
    println!("cargo:rustc-link-lib=uiohook");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xtst");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", libuiohook_dir.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum("_event_type")
        .rustified_enum("mouse_button")
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_ord(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();
    build
        .include(libuiohook_dir.join("include"))
        .include(libuiohook_dir.join("src"))
        .file(libuiohook_dir.join("src/logger.c"));

    if cfg!(target_os = "linux") {
        build
            .file(libuiohook_dir.join("src/x11/input_hook.c"))
            .file(libuiohook_dir.join("src/x11/post_event.c"))
            .file(libuiohook_dir.join("src/x11/system_properties.c"))
            .file(libuiohook_dir.join("src/x11/input_helper.c"));
    } else if cfg!(target_os = "macos") {
        build
            .file(libuiohook_dir.join("src/darwin/input_hook.c"))
            .file(libuiohook_dir.join("src/darwin/post_event.c"))
            .file(libuiohook_dir.join("src/darwin/system_properties.c"))
            .file(libuiohook_dir.join("src/darwin/input_helper.c"));
    } else if cfg!(target_os = "windows") {
        build
            .file(libuiohook_dir.join("src/windows/input_hook.c"))
            .file(libuiohook_dir.join("src/windows/post_event.c"))
            .file(libuiohook_dir.join("src/windows/system_properties.c"))
            .file(libuiohook_dir.join("src/windows/input_helper.c"));
    }

    build.compile("uiohook");
}