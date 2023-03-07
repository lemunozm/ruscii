//! # Keyboard
//!
//! The `keyboard` module contains all implementation-related details of keyboard I/O and the
//! key event interface.
//!
//! ## Example
//!
//! Most applications will interact with the key event interface through its
//! [`State`](crate::app::State) object.
//!
//! `Pressed` and `Released` events can be retrieved by use of the [`Keyboard::last_key_events`]
//! function while held-down keys can be obtained by the [`Keyboard::get_keys_down`] function. These
//! can then be pattern-matched to find the type of key as in the following example:
//!
//! ```rust,ignore
//! # use ruscii::app::{App, State};
//! # use ruscii::keyboard::{Key, KeyEvent};
//! # use ruscii::terminal::Window;
//! #
//! let mut app = App::new();
//!
//! app.run(|app_state: &mut State, window: &mut Window| {
//!     for key_event in app_state.keyboard().last_key_events() {
//!         match key_event {
//!             KeyEvent::Pressed(Key::Esc) => app_state.stop(),
//!             KeyEvent::Pressed(Key::Q) => app_state.stop(),
//!             _ => (),
//!         }
//!     }
//!
//!     for key_down in app_state.keyboard().get_keys_down() {
//!         match key_down {
//!             Key::W => state.left_player.direction = -1,
//!             Key::S => state.left_player.direction = 1,
//!             Key::O => state.right_player.direction = -1,
//!             Key::L => state.right_player.direction = 1,
//!             _ => (),
//!         }
//!     }
//! });
//! ```

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crossterm as ct;
use device_query as dq;
use dq::DeviceQuery;

const KEY_EVENT_FOCUS_DELAY_MS: u64 = 20;

/// The keys detectable by `ruscii`.
///
/// A value, [`Key::Unknown`], is provided when a key is detected but the type of key cannot be
/// ascertained.
///
/// ## Exceptional Behavior
///
/// Certain values of this enum represent _positions_ on the keyboard rather than the key type.
/// These values might behave differently than expected when using different keyboard layouts.
/// This includes:
///
/// - grave/backtick `` ` ``
/// - minus `-`
/// - equal `=`
/// - left and right bracket `[]`
/// - forward and back slash `/\ `
/// - semicolon `;`
/// - apostrophe `'`
/// - comma `,`
/// - dot `.`
///
/// These keys are named according to their function in a U.S. ASCII keyboard layout. Differing
/// layouts may vary in which key event is fired.
///
/// A differing layout may also affect how key events are fired; certain keyboards may generate
/// only one event by pressing two keys.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Esc,
    Space,
    Enter,
    Backspace,
    CapsLock,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // The following keys names represent the position of the key in a US keyboard, can vary in others keyboards.
    // Some keys may generate only one event by two key press depending the keyboard distribution.
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    BackSlash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,

    Unknown,
}

/* TODO: Add modifiers to events.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Modifier {
    LControl, RControl, LShift, RShift, LAlt, RAlt, Meta,
}
*/

/// Events that are detected for each key.
///
/// May exhibit unintended behavior depending on keyboard layout. This behavior is documented
/// in [`Key`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

impl KeyEvent {
    /// If the [`KeyEvent`] is [`KeyEvent::Pressed`], returns the [`Key`] wrapped by the event and
    /// otherwise [`None`].
    pub fn pressed(self) -> Option<Key> {
        if let KeyEvent::Pressed(key) = self {
            Some(key)
        } else {
            None
        }
    }

    /// If the [`KeyEvent`] is [`KeyEvent::Released`], returns the [`Key`] wrapped by the event and
    /// otherwise [`None`].
    pub fn released(self) -> Option<Key> {
        if let KeyEvent::Released(key) = self {
            Some(key)
        } else {
            None
        }
    }
}

/// An object representing the state of the keyboard.
pub struct Keyboard {
    thread_running: Arc<AtomicBool>,
    acc_thread: Option<JoinHandle<()>>,
    event_thread: Option<JoinHandle<()>>,
    event_receiver: Receiver<KeyEvent>,
    state: HashMap<Key, usize>,
    last_key_events: Vec<KeyEvent>,
    last_key_stamp: usize,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let thread_running = Arc::new(AtomicBool::new(true));

        let (acc_sender, acc_receiver): (Sender<KeyEvent>, Receiver<KeyEvent>) = mpsc::channel();
        let (event_sender, event_receiver): (Sender<KeyEvent>, Receiver<KeyEvent>) =
            mpsc::channel();

        let acc_thread_running = thread_running.clone();
        let pressed_event_sender = event_sender.clone();
        let acc_thread = thread::spawn(move || {
            let mut event_accumulator: Vec<(KeyEvent, Instant)> = vec![];
            let mut last_input_timestamp =
                Instant::now() - Duration::from_millis(KEY_EVENT_FOCUS_DELAY_MS + 1);
            while acc_thread_running.load(Ordering::SeqCst) {
                if let Some(timestamp) = Self::process_input_timestamp() {
                    last_input_timestamp = timestamp;
                }

                event_accumulator.retain(|(key_event, instant)| {
                    if Instant::now() - *instant > Duration::from_millis(KEY_EVENT_FOCUS_DELAY_MS) {
                        return false;
                    }

                    if let Some(duration) = instant.checked_duration_since(last_input_timestamp) {
                        if duration <= Duration::from_millis(KEY_EVENT_FOCUS_DELAY_MS) {
                            pressed_event_sender.send(*key_event).unwrap();
                            return false;
                        }
                    }

                    if let Some(duration) = last_input_timestamp.checked_duration_since(*instant) {
                        if duration <= Duration::from_millis(KEY_EVENT_FOCUS_DELAY_MS) {
                            pressed_event_sender.send(*key_event).unwrap();
                            return false;
                        }
                    }

                    true
                });

                match acc_receiver.recv_timeout(Duration::from_millis(1)) {
                    Ok(key_event) => event_accumulator.push((key_event, Instant::now())),
                    Err(mpsc::RecvTimeoutError::Timeout) => (),
                    Err(_) => (), //it is reached maybe because of rust issue #39364
                };
            }
        });

        let event_thread_running = thread_running.clone();
        let released_event_sender = event_sender.clone();
        let event_thread = thread::spawn(move || {
            let device = dq::DeviceState::new();
            let mut last_device_state = Vec::new();
            while event_thread_running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(1));
                let new_device_state = device.get_keys();
                Self::process_pressed_event(&acc_sender, &new_device_state, &last_device_state);
                Self::process_released_event(
                    &released_event_sender,
                    &new_device_state,
                    &last_device_state,
                );
                std::mem::replace(&mut last_device_state, new_device_state);
            }
        });

        Keyboard {
            thread_running,
            event_thread: Some(event_thread),
            acc_thread: Some(acc_thread),
            event_receiver,
            state: HashMap::new(),
            last_key_events: Vec::new(),
            last_key_stamp: 0,
        }
    }

    /// Retrieves all the [`KeyEvents`] that were fired during the previous frame.
    pub fn last_key_events(&self) -> &Vec<KeyEvent> {
        &self.last_key_events
    }

    /// Retrieves all the currently held down [`Key`]s.
    pub fn get_keys_down(&self) -> Vec<Key> {
        let mut keys = self.state.iter().collect::<Vec<_>>();
        keys.sort_by(|key_stamp_a, key_stamp_b| key_stamp_a.1.cmp(key_stamp_b.1));
        keys.into_iter().map(|x| *x.0).collect()
    }

    /// Clears the [`KeyEvents`] from the last frame and consumes new ones from the event
    /// [`Receiver`].
    pub fn consume_key_events(&mut self) -> &Vec<KeyEvent> {
        self.last_key_events.clear();
        let events = self.event_receiver.try_iter().collect::<Vec<_>>();

        for event in &events {
            if let KeyEvent::Pressed(key) = *event {
                if !self.state.contains_key(&key) {
                    self.state.insert(key, self.last_key_stamp);
                    self.last_key_events.push(*event);
                    self.last_key_stamp += 1;
                }
            }
        }

        for event in &events {
            if let KeyEvent::Released(key) = *event {
                if self.state.contains_key(&key) {
                    self.state.remove(&key);
                    self.last_key_events.push(*event);
                }
            }
        }
        &self.last_key_events
    }

    fn process_input_timestamp() -> Option<Instant> {
        let mut input_received = false;
        while ct::event::poll(Duration::from_millis(0)).unwrap() {
            //means: has the app the focus?
            ct::event::read().unwrap();
            input_received = true;
        }

        if input_received {
            Some(Instant::now())
        } else {
            None
        }
    }

    fn process_pressed_event(
        sender: &Sender<KeyEvent>,
        new_state: &Vec<dq::Keycode>,
        last_state: &Vec<dq::Keycode>,
    ) {
        let pressed: Vec<dq::Keycode> = new_state
            .clone()
            .into_iter()
            .filter(|x| !last_state.contains(x))
            .collect();
        for keycode in pressed {
            let key = Self::transform_device_key(&keycode);
            if key != Key::Unknown {
                sender.send(KeyEvent::Pressed(key)).unwrap();
            }
        }
    }

    fn process_released_event(
        sender: &Sender<KeyEvent>,
        new_state: &Vec<dq::Keycode>,
        last_state: &Vec<dq::Keycode>,
    ) {
        let released: Vec<dq::Keycode> = last_state
            .clone()
            .into_iter()
            .filter(|x| !new_state.contains(x))
            .collect();
        for keycode in released {
            let key = Self::transform_device_key(&keycode);
            if key != Key::Unknown {
                sender.send(KeyEvent::Released(key)).unwrap();
            }
        }
    }

    /// Converts a [`dq::Keycode`] to the corresponding [`Key`]. Unhandled keycodes are converted to
    /// [`Key::Unknown`].
    fn transform_device_key(device_key: &dq::Keycode) -> Key {
        match device_key {
            dq::Keycode::Escape => Key::Esc,
            dq::Keycode::Enter => Key::Enter,
            dq::Keycode::Space => Key::Space,

            dq::Keycode::Up => Key::Up,
            dq::Keycode::Down => Key::Down,
            dq::Keycode::Left => Key::Left,
            dq::Keycode::Right => Key::Right,

            dq::Keycode::Backspace => Key::Backspace,
            dq::Keycode::CapsLock => Key::CapsLock,
            dq::Keycode::Tab => Key::Tab,
            dq::Keycode::Home => Key::Home,
            dq::Keycode::End => Key::End,
            dq::Keycode::PageUp => Key::PageUp,
            dq::Keycode::PageDown => Key::PageDown,
            dq::Keycode::Insert => Key::Insert,
            dq::Keycode::Delete => Key::Delete,

            dq::Keycode::A => Key::A,
            dq::Keycode::B => Key::B,
            dq::Keycode::C => Key::C,
            dq::Keycode::D => Key::D,
            dq::Keycode::E => Key::E,
            dq::Keycode::F => Key::F,
            dq::Keycode::G => Key::G,
            dq::Keycode::H => Key::H,
            dq::Keycode::I => Key::I,
            dq::Keycode::J => Key::J,
            dq::Keycode::K => Key::K,
            dq::Keycode::L => Key::L,
            dq::Keycode::M => Key::M,
            dq::Keycode::N => Key::N,
            dq::Keycode::O => Key::O,
            dq::Keycode::P => Key::P,
            dq::Keycode::Q => Key::Q,
            dq::Keycode::R => Key::R,
            dq::Keycode::S => Key::S,
            dq::Keycode::T => Key::T,
            dq::Keycode::U => Key::U,
            dq::Keycode::V => Key::V,
            dq::Keycode::W => Key::W,
            dq::Keycode::X => Key::X,
            dq::Keycode::Y => Key::Y,
            dq::Keycode::Z => Key::Z,

            dq::Keycode::Key0 => Key::Num0,
            dq::Keycode::Key1 => Key::Num1,
            dq::Keycode::Key2 => Key::Num2,
            dq::Keycode::Key3 => Key::Num3,
            dq::Keycode::Key4 => Key::Num4,
            dq::Keycode::Key5 => Key::Num5,
            dq::Keycode::Key6 => Key::Num6,
            dq::Keycode::Key7 => Key::Num7,
            dq::Keycode::Key8 => Key::Num8,
            dq::Keycode::Key9 => Key::Num9,

            dq::Keycode::F1 => Key::F1,
            dq::Keycode::F2 => Key::F2,
            dq::Keycode::F3 => Key::F3,
            dq::Keycode::F4 => Key::F4,
            dq::Keycode::F5 => Key::F5,
            dq::Keycode::F6 => Key::F6,
            dq::Keycode::F7 => Key::F7,
            dq::Keycode::F8 => Key::F8,
            dq::Keycode::F9 => Key::F9,
            dq::Keycode::F10 => Key::F10,
            dq::Keycode::F11 => Key::F11,
            dq::Keycode::F12 => Key::F12,

            dq::Keycode::Grave => Key::Grave,
            dq::Keycode::Minus => Key::Minus,
            dq::Keycode::Equal => Key::Equal,
            dq::Keycode::LeftBracket => Key::LeftBracket,
            dq::Keycode::RightBracket => Key::RightBracket,
            dq::Keycode::BackSlash => Key::BackSlash,
            dq::Keycode::Semicolon => Key::Semicolon,
            dq::Keycode::Apostrophe => Key::Apostrophe,
            dq::Keycode::Comma => Key::Comma,
            dq::Keycode::Dot => Key::Dot,
            dq::Keycode::Slash => Key::Slash,

            _ => Key::Unknown,
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.thread_running.store(false, Ordering::SeqCst);
        self.acc_thread.take().unwrap().join().unwrap();
        self.event_thread.take().unwrap().join().unwrap();
    }
}
