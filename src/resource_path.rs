use std::env::current_exe;
use std::path::{Path, PathBuf};

pub fn resource_path(filename: &str) -> PathBuf {
    let exe_path = current_exe().expect("Could not get executable path");
    let bundled_path = exe_path
        .parent().unwrap()
        .parent().unwrap()
        .join("Resources")
        .join(filename);

    if bundled_path.exists() {
        return bundled_path;
    }

    // Not running from inside a .app bundle (e.g. `cargo run` during development) -
    // fall back to the asset's location in the source tree.
    Path::new(env!("CARGO_MANIFEST_DIR")).join(filename)
}