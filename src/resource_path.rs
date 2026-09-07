use std::env::current_exe;
use std::path::PathBuf;

pub fn resource_path(filename: &str) -> PathBuf {
    let exe_path = current_exe().expect("Could not get executable path");
    exe_path
        .parent().unwrap()
        .parent().unwrap()
        .join("Resources")
        .join(filename)
}