use std::{time, thread, sync::{Arc, Mutex}};
use rdev::{Event, EventType, Key};
use crate::{ranker, rdev_keymap};

// In test builds we never want to hit the real OS keyboard (rdev::simulate would type into
// whatever window has focus while `cargo test` runs), so key sends are swapped for an
// in-memory recorder that the tests below can inspect and replay.
#[cfg(test)]
thread_local! {
    static TEST_SENT_EVENTS: std::cell::RefCell<Vec<EventType>> = std::cell::RefCell::new(Vec::new());
}

#[cfg(not(test))]
fn send_key(event_type: &EventType) -> bool {
    rdev::simulate(event_type).is_ok()
}

#[cfg(test)]
fn send_key(event_type: &EventType) -> bool {
    TEST_SENT_EVENTS.with(|log| log.borrow_mut().push(*event_type));
    true
}

// Sends a key and, only if the OS actually accepted it, records that we owe ourselves one more
// echo of it back through word_correction. See the comment on `pending_synthetic_events` in
// word_correction for why this has to be a count rather than a plain "are we correcting" flag.
fn send_and_track(event_type: &EventType, pending_synthetic_events: &Arc<Mutex<usize>>) {
  if send_key(event_type) {
    *pending_synthetic_events.lock().unwrap() += 1;
  }
}

pub fn word_correction(
  event: Event,
  user_word: &Arc<Mutex<String>>,
  spellcheck_enabled: &Arc<Mutex<bool>>,
  last_correction: &Arc<Mutex<Option<(String, String)>>>,
  // Counts synthetic key events we have posted via rdev::simulate() that we still expect to
  // see come back through this very callback (rdev::listen's tap sees our own injected
  // keystrokes too). It is NOT a "currently correcting" flag: on macOS, simulate() posts the
  // event into the OS's HID event queue but does not deliver it back to our tap synchronously,
  // so any injected key is only observed here well after perform_correction/revert_correction
  // has already returned. A plain bool that gets reset once we're done *sending* goes back to
  // false before those echoes arrive, so they get misread as real user input. Counting exactly
  // how many of our own events are still outstanding, and decrementing one per echo instead of
  // resetting on a timer/code-position basis, keeps this correct regardless of when the OS
  // actually delivers them back to us.
  pending_synthetic_events: &Arc<Mutex<usize>>
) {

  {
    let mut pending = pending_synthetic_events.lock().unwrap();
    if *pending > 0 {
      *pending -= 1;
      return; // this is an echo of our own simulated keystroke, not real input
    }
  }

  if !*spellcheck_enabled.lock().unwrap() {
    return;
  }

  if event.event_type == EventType::KeyPress(Key::Backspace) {
    let mut correction = last_correction.lock().unwrap();

    if let Some((original, corrected)) = correction.take() {
        // auto-correction made wrong word -> backspace to undo
        revert_correction(&corrected, &original, &pending_synthetic_events);

        let mut buffer = user_word.lock().unwrap();
        *buffer = original;
    } else {
        // normal backspace
        let mut buffer = user_word.lock().unwrap();
        buffer.pop();
    }

    return;
    }

  match event.name {
    Some(user_char) => {
      if !user_char.chars().all(char::is_alphabetic) {
        let mut buffer = user_word.lock().unwrap();
        let original_word = buffer.clone();
        let corrected_word: String = ranker::handle_completed_word(&original_word.to_string());

        if original_word != corrected_word {
          perform_correction(&original_word, &corrected_word, &pending_synthetic_events);
          *last_correction.lock().unwrap() = Some((original_word.clone(), corrected_word.clone()));
        }
        else {
          *last_correction.lock().unwrap() = None;
        }

        buffer.clear();

      }
      else {
        let mut buffer = user_word.lock().unwrap();

        if buffer.is_empty() {
          *last_correction.lock().unwrap() = None;
        }

        buffer.push_str(&user_char);
      }
    },
    None => (),
  }
}

pub fn perform_correction(original_word: &str, corrected_word: &str, pending_synthetic_events: &Arc<Mutex<usize>>) {
  let delay = time::Duration::from_millis(5); // gives OS queue adequate amount of time to process simulated key press

  for _ in 0..original_word.chars().count()+1 {
    send_and_track(&EventType::KeyPress(Key::Backspace), pending_synthetic_events);
    send_and_track(&EventType::KeyRelease(Key::Backspace), pending_synthetic_events);
    thread::sleep(delay);
  }

  for character in corrected_word.chars() {
    if let Some(rdev_key) = rdev_keymap::char_to_key(character) {
      send_and_track(&EventType::KeyPress(rdev_key), pending_synthetic_events);
      send_and_track(&EventType::KeyRelease(rdev_key), pending_synthetic_events);
      thread::sleep(delay);
    }
  }

  send_and_track(&EventType::KeyPress(Key::Space), pending_synthetic_events);
  send_and_track(&EventType::KeyRelease(Key::Space), pending_synthetic_events);
}

pub fn revert_correction(corrected_word: &str, original_word: &str, pending_synthetic_events: &Arc<Mutex<usize>>) {
  let delay = time::Duration::from_millis(5);
  // delete corrected word and revert to original

  for _ in 0..corrected_word.chars().count() {
    send_and_track(&EventType::KeyPress(Key::Backspace), pending_synthetic_events);
    send_and_track(&EventType::KeyRelease(Key::Backspace), pending_synthetic_events);
    thread::sleep(delay);
  }

  for character in original_word.chars() {
    if let Some(rdev_key) = rdev_keymap::char_to_key(character) {
      send_and_track(&EventType::KeyPress(rdev_key), pending_synthetic_events);
      send_and_track(&EventType::KeyRelease(rdev_key), pending_synthetic_events);
      thread::sleep(delay);
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn char_event(c: char) -> Event {
        Event {
            time: SystemTime::now(),
            name: Some(c.to_string()),
            event_type: EventType::KeyPress(rdev_keymap::char_to_key(c).unwrap()),
        }
    }

    fn space_event() -> Event {
        Event {
            time: SystemTime::now(),
            name: Some(" ".to_string()),
            event_type: EventType::KeyPress(Key::Space),
        }
    }

    fn backspace_event() -> Event {
        Event {
            time: SystemTime::now(),
            name: None,
            event_type: EventType::KeyPress(Key::Backspace),
        }
    }

    fn drain_sent_events() -> Vec<EventType> {
        TEST_SENT_EVENTS.with(|log| log.borrow_mut().drain(..).collect())
    }

    struct Harness {
        user_word: Arc<Mutex<String>>,
        spellcheck_enabled: Arc<Mutex<bool>>,
        last_correction: Arc<Mutex<Option<(String, String)>>>,
        pending_synthetic_events: Arc<Mutex<usize>>,
    }

    impl Harness {
        fn new() -> Self {
            TEST_SENT_EVENTS.with(|log| log.borrow_mut().clear());
            Harness {
                user_word: Arc::new(Mutex::new(String::new())),
                spellcheck_enabled: Arc::new(Mutex::new(true)),
                last_correction: Arc::new(Mutex::new(None)),
                pending_synthetic_events: Arc::new(Mutex::new(0)),
            }
        }

        fn send(&self, event: Event) {
            word_correction(
                event,
                &self.user_word,
                &self.spellcheck_enabled,
                &self.last_correction,
                &self.pending_synthetic_events,
            );
        }

        fn pending(&self) -> usize {
            *self.pending_synthetic_events.lock().unwrap()
        }

        fn type_word(&self, word: &str) {
            for c in word.chars() {
                self.send(char_event(c));
            }
        }
    }

    #[test]
    fn perform_correction_leaves_events_pending_instead_of_clearing_immediately() {
        let h = Harness::new();

        h.type_word("teh");
        assert_eq!(h.pending(), 0);

        // space triggers the correction: "teh" -> "the"
        h.send(space_event());

        // Unlike the old bool flag (which was already reset to false at this point),
        // the counter stays above zero: we are still owed an echo for every key we
        // just posted (backspaces to delete "teh" + retype of "the" + trailing space).
        assert!(h.pending() > 0, "expected outstanding synthetic events to still be tracked");
        assert!(h.last_correction.lock().unwrap().is_some());
    }

    #[test]
    fn echoed_synthetic_keys_are_not_misread_as_user_input() {
        let h = Harness::new();

        h.type_word("teh");
        h.send(space_event()); // triggers perform_correction: "teh" -> "the"

        let sent = drain_sent_events();
        let expected_pending = sent.len();
        assert_eq!(h.pending(), expected_pending);
        assert_eq!(sent[0], EventType::KeyPress(Key::Backspace));

        // Replay every key perform_correction just sent, exactly as rdev's listener would once
        // the OS run loop resumes and delivers them — including the backspaces that, under the
        // old bool flag, were misread as the user rejecting the correction.
        for event_type in sent {
            h.send(Event { time: SystemTime::now(), name: None, event_type });
        }

        assert_eq!(h.pending(), 0, "all echoes should have been drained");
        assert!(
            h.last_correction.lock().unwrap().is_some(),
            "echoes of our own correction must not consume last_correction as if the user hit backspace"
        );
        assert!(h.user_word.lock().unwrap().is_empty());
    }

    #[test]
    fn real_backspace_after_echoes_drain_still_reverts_the_correction() {
        let h = Harness::new();

        h.type_word("teh");
        h.send(space_event());
        for event_type in drain_sent_events() {
            h.send(Event { time: SystemTime::now(), name: None, event_type });
        }
        assert_eq!(h.pending(), 0);
        assert!(h.last_correction.lock().unwrap().is_some());

        // Now the user genuinely presses backspace to reject the correction.
        h.send(backspace_event());

        assert!(h.last_correction.lock().unwrap().is_none());
        assert_eq!(*h.user_word.lock().unwrap(), "teh");
        // revert_correction should have queued its own outstanding echoes rather than
        // resetting anything prematurely.
        assert!(h.pending() > 0);
    }
}
