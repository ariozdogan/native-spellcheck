pub mod dictionary;
pub mod edit_distance;
pub mod ranker;
pub mod keyboard_map;
pub mod edit_cost;
pub mod word_correction;
pub mod rdev_keymap;
pub mod tray;
pub mod resource_path;

pub fn placeholder() -> String {
  "lib is wired up".to_string()
}