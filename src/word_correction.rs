use std::{time, thread, sync::{Arc, Mutex}};
use rdev::{Event, simulate, EventType, Key};
use crate::{ranker, rdev_keymap};


pub fn word_correction(event: Event, user_word: &Arc<Mutex<String>>, spellcheck_enabled: &Arc<Mutex<bool>>) {
  if !*spellcheck_enabled.lock().unwrap() {
    return;
  }

  match event.name {
    Some(user_char) => {
      if !user_char.chars().all(char::is_alphabetic) {
        let mut buffer = user_word.lock().unwrap();
        let original_word = buffer.clone();
        let corrected_word: String = ranker::handle_completed_word(&original_word.to_string());

        if original_word != corrected_word {
          perform_correction(&original_word, &corrected_word);
        }

        buffer.clear();
      }
      else {
        user_word.lock().unwrap().push_str(&user_char);
      }
    },
    None => (),
  }
}

pub fn perform_correction(original_word: &str, corrected_word: &str) {
  let delay = time::Duration::from_millis(5); // gives OS queue adequate amount of time to process simulated key press

  for char in 0..original_word.chars().count()+1 {
    simulate(&EventType::KeyPress(Key::Backspace)).ok();
    simulate(&EventType::KeyRelease(Key::Backspace)).ok();
    thread::sleep(delay);
  }

  for char in corrected_word.chars() {
    if let Some(rdev_key) = rdev_keymap::char_to_key(char) {
      simulate(&EventType::KeyPress(rdev_key)).ok();
      simulate(&EventType::KeyRelease(rdev_key)).ok();
      thread::sleep(delay);
    }
  }

  simulate(&EventType::KeyPress(Key::Space)).ok();
  simulate(&EventType::KeyRelease(Key::Space)).ok();
}