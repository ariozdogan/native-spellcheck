use tray_icon::Icon;


pub fn build_tray_icon() -> Icon {
  let path: String = String::from("assets/icon.png");
  let img = image::open(path)
    .expect("Failed to open png")
    .into_rgba8();
  
  let (width, height) = img.dimensions();
  let rgba_bytes = img.into_raw();

  Icon::from_rgba(rgba_bytes, width, height)
    .expect("Failed to create icon from rgba bytes")
}
