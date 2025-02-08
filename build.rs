use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libuiohook_dir = root.join("libuiohook");

    println!("cargo:rustc-link-search={}", libuiohook_dir.display());
    println!("cargo:rustc-link-lib=uiohook");
    println!("cargo:rerun-if-changed=wrapper.h");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let mut build = cc::Build::new();
    build
        .include(&libuiohook_dir.join("include"))
        .include(&libuiohook_dir.join("src"))
        .file(libuiohook_dir.join("src/logger.c"));

    match target_os.as_str() {
        "linux" => {
            // Find and link X11 libraries
            pkg_config::probe_library("x11").unwrap();
            // pkg_config::probe_library("xext").unwrap();
            // pkg_config::probe_library("xi").unwrap();
            pkg_config::probe_library("xtst").unwrap();
            // pkg_config::probe_library("xkbcommon").unwrap();
            build
                .file(libuiohook_dir.join("src/x11/input_hook.c"))
                .file(libuiohook_dir.join("src/x11/post_event.c"))
                .file(libuiohook_dir.join("src/x11/system_properties.c"))
                .file(libuiohook_dir.join("src/x11/input_helper.c"));
        }
        "macos" => {
            build
                .file(libuiohook_dir.join("src/darwin/input_hook.c"))
                .file(libuiohook_dir.join("src/darwin/post_event.c"))
                .file(libuiohook_dir.join("src/darwin/system_properties.c"))
                .file(libuiohook_dir.join("src/darwin/input_helper.c"));

            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=ApplicationServices");
        }
        "windows" => {
            build
                .file(libuiohook_dir.join("src/windows/input_hook.c"))
                .file(libuiohook_dir.join("src/windows/post_event.c"))
                .file(libuiohook_dir.join("src/windows/system_properties.c"))
                .file(libuiohook_dir.join("src/windows/input_helper.c"));

            println!("cargo:rustc-link-lib=user32");
        }
        _ => panic!("Unsupported operating system"),
    }

    build
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-pragmas")
        .compile("uiohook");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", libuiohook_dir.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum("_event_type")
        .rustified_enum("mouse_button")
        .derive_debug(true)
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
}
