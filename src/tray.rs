use tray_icon::{TrayIconBuilder, TrayIconEvent, menu::{Menu, Icon, MenuEvent}};


pub fn build_tray_icon() {
    let tray_menu = Menu::new();

    let tray_icon = TrayIconBuilder::new() // creates tray icon with a menu
    .with_menu(Box::new(tray_menu))
    .with_tooltip("system-tray - tray icon library")
    .with_icon(icon)
    .build()
    .unwrap();

    if let Ok(event) = TrayIconEvent::receiver().try_recv() {
        println!("tray event: {:?}", event);
    }

    if let Ok(event) = MenuEvent::receiver().try_recv() {
        println!("menu event: {:?}", event);
    }   
}

