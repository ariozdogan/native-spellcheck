use tray_icon::Icon;
use crate::resource_path;

pub fn build_tray_icon() -> Icon {
  let icon_path = resource_path::resource_path("assets/tray-icon.png");
  let img = image::open(icon_path)
    .expect("Failed to open png")
    .into_rgba8();
  
  let (width, height) = img.dimensions();
  let rgba_bytes = img.into_raw();

  Icon::from_rgba(rgba_bytes, width, height)
    .expect("Failed to create icon from rgba bytes")
}
