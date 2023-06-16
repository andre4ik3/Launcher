use std::env;

fn main() {
    if env::var("CARGO_FEATURE_FFI_SWIFT").is_ok() {
        println!("cargo:rerun-if-env-changed=CONFIGURATION");
        println!("cargo:rerun-if-changed=src/ffi.rs");
        swift_bridge_build::parse_bridges(vec!["src/ffi.rs"])
            .write_all_concatenated("SwiftBridge", env!("CARGO_PKG_NAME"));
    }
}
