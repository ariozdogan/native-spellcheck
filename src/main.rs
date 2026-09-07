use std::{thread, sync::{Arc, Mutex}};
use rdev::{listen, Event};
use tray_icon::{TrayIconBuilder, TrayIconEvent, Icon, menu::{Menu, MenuEvent, CheckMenuItem}};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use spellcheck::{tray, word_correction};

fn main() {
  let user_word: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
  let user_word_clone = Arc::clone(&user_word);

  let spellcheck_enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
  let spellcheck_enabled_clone = Arc::clone(&spellcheck_enabled);

  let last_correction: Arc<Mutex<Option<(String, String)>>> = Arc::new(Mutex::new(None));
  let last_correction_clone = Arc::clone(&last_correction);

  let pending_synthetic_events: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
  let pending_synthetic_events_clone = Arc::clone(&pending_synthetic_events);

  enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
  }

  thread::spawn(move || {
    let callback = move |event: Event| {
      word_correction::word_correction(
        event, 
        &user_word_clone, 
        &spellcheck_enabled_clone, 
        &last_correction_clone,
        &pending_synthetic_events_clone);
    };

    if let Err(error) = listen(callback) {
      println!("Error: {:?}", error)
    }
  });

  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
  let proxy = event_loop.create_proxy(); // allows tray_icon events to be sent into tao loop

  let proxy_tray = proxy.clone();
  TrayIconEvent::set_event_handler(Some(move |event| {
    proxy_tray.send_event(UserEvent::TrayIconEvent(event)).ok();
  }));

  let proxy_menu = proxy.clone();
  MenuEvent::set_event_handler(Some(move |event| {
    proxy_menu.send_event(UserEvent::MenuEvent(event)).ok();
  }));


  let tray_menu: Menu = Menu::new();

  let toggle_spellcheck = CheckMenuItem::new(
    "Enable spellcheck", // label
    true, // clickable
    false, // initial state: off
    None  // no keyboard shortcut
  );

  tray_menu.append(&toggle_spellcheck).unwrap();

  let tray_icon: Icon = tray::build_tray_icon();

  let _tray = TrayIconBuilder::new() // creates tray icon with a menu
    .with_menu(Box::new(tray_menu))
    .with_tooltip("system-tray - tray icon library")
    .with_icon(tray_icon)
    .build()
    .unwrap();


  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      tao::event::Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
        println!("tray event, {:?}", event);
      }
      tao::event::Event::UserEvent(UserEvent::MenuEvent(event)) => {
        println!("menu event, {:?}", event);

        if event.id == toggle_spellcheck.id() {
          let is_checked = toggle_spellcheck.is_checked();
          let mut enabled = spellcheck_enabled.lock().unwrap();
          *enabled = is_checked;
        }
      }
      _ => (),
    }
  });
}




