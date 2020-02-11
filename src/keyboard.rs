use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc};
use std::thread::{self, JoinHandle};
use std::collections::{HashMap};
use std::time;

use crossterm as ct;
use device_query as dq;
use dq::DeviceQuery;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Esc, Space, Enter,
    Up, Down, Left, Right,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

impl KeyEvent {
    pub fn pressed(self) -> Option<Key> {
       if let KeyEvent::Pressed(key) = self { Some(key) } else { None }
    }

    pub fn released(self) -> Option<Key> {
       if let KeyEvent::Released(key) = self { Some(key) } else { None }
    }
}

pub struct Keyboard {
    stdin_thread: Option<JoinHandle<()>>,
    device_thread: Option<JoinHandle<()>>,
    event_receiver: Receiver<KeyEvent>,
    threads_running: Arc<AtomicBool>,
    state: HashMap<Key, usize>,
    last_key_events: Vec<KeyEvent>,
    last_key_stamp: usize,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let threads_running = Arc::new(AtomicBool::new(true));
        let (sender, receiver): (Sender<KeyEvent>, Receiver<KeyEvent>) = mpsc::channel();

        let stdin_sender = sender.clone();
        let stdin_running = threads_running.clone();
        let stdin_thread = thread::spawn(move || {
            while stdin_running.load(Ordering::SeqCst) {
                Self::process_input_event(&stdin_sender);
            }
        });

        let device_sender = sender.clone();
        let device_running = threads_running.clone();
        let device_thread = thread::spawn(move || {
            let device = dq::DeviceState::new();
            let mut last_device_state = Vec::new();
            while device_running.load(Ordering::SeqCst) {
                Self::process_device_event(&device_sender, &device, &mut last_device_state);
            }
        });

        Keyboard {
            stdin_thread: Some(stdin_thread),
            device_thread: Some(device_thread),
            event_receiver: receiver,
            threads_running,
            state: HashMap::new(),
            last_key_events: Vec::new(),
            last_key_stamp: 0,
        }
    }

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

    pub fn last_key_events(&self) -> &Vec<KeyEvent> {
        &self.last_key_events
    }

    pub fn get_keys_down(&self) -> Vec<Key> {
        let mut keys = self.state.iter().collect::<Vec<_>>();
        keys.sort_by(|a, b| a.1.cmp(b.1));
        keys.into_iter().map(|x| *x.0).collect()
    }

    fn process_input_event(sender: &Sender<KeyEvent>) {
        if ct::event::poll(time::Duration::from_millis(1)).unwrap() {
            let event = ct::event::read().unwrap();
            if let ct::event::Event::Key(key_event) = event {
                let key = Self::transform_input_key(&key_event.code);
                if key != Key::Unknown {
                    sender.send(KeyEvent::Pressed(key)).unwrap();
                }
            }
        }
    }

    fn process_device_event(sender: &Sender<KeyEvent>, device: &dq::DeviceState, last_device_state: &mut Vec<dq::Keycode>) {
        let new_state = device.get_keys();
        let released: Vec<dq::Keycode> = last_device_state.clone().into_iter().filter(|x| !new_state.contains(x)).collect();
        std::mem::replace(last_device_state, new_state);
        for keycode in released {
            let key = Self::transform_device_key(&keycode);
            if key != Key::Unknown {
                sender.send(KeyEvent::Released(key)).unwrap();
            }
        }
        thread::sleep(time::Duration::from_millis(1));
    }

    fn transform_input_key(input_key: &ct::event::KeyCode) -> Key {
        match input_key {
            ct::event::KeyCode::Enter => Key::Enter,
            ct::event::KeyCode::Esc => Key::Esc,
            ct::event::KeyCode::Up => Key::Up,
            ct::event::KeyCode::Down => Key::Down,
            ct::event::KeyCode::Left => Key::Left,
            ct::event::KeyCode::Right => Key::Right,
            ct::event::KeyCode::Char(c) => match c {
                ' ' => Key::Space,
                'a' => Key::A,
                'b' => Key::B,
                'c' => Key::C,
                'd' => Key::D,
                'e' => Key::E,
                'f' => Key::F,
                'g' => Key::G,
                'h' => Key::H,
                'i' => Key::I,
                'j' => Key::J,
                'k' => Key::K,
                'l' => Key::L,
                'm' => Key::M,
                'n' => Key::N,
                'o' => Key::O,
                'p' => Key::P,
                'q' => Key::Q,
                'r' => Key::R,
                's' => Key::S,
                't' => Key::T,
                'u' => Key::U,
                'v' => Key::V,
                'w' => Key::W,
                'x' => Key::X,
                'y' => Key::Y,
                'z' => Key::Z,
                '0' => Key::Num0,
                '1' => Key::Num1,
                '2' => Key::Num2,
                '3' => Key::Num3,
                '4' => Key::Num4,
                '5' => Key::Num5,
                '6' => Key::Num6,
                '7' => Key::Num7,
                '8' => Key::Num8,
                '9' => Key::Num9,
                _ => Key::Unknown
            },
            ct::event::KeyCode::F(n) => match n {
                1 => Key::F1,
                2 => Key::F2,
                3 => Key::F3,
                4 => Key::F4,
                5 => Key::F5,
                6 => Key::F6,
                7 => Key::F7,
                8 => Key::F8,
                9 => Key::F9,
                10 => Key::F10,
                11 => Key::F11,
                12 => Key::F12,
                _ => unreachable!()
            },
            _ => Key::Unknown,
        }
    }

    fn transform_device_key(device_key: &dq::Keycode) -> Key {
        match device_key {
            dq::Keycode::Escape => Key::Esc,
            dq::Keycode::Enter => Key::Enter,
            dq::Keycode::Up => Key::Up,
            dq::Keycode::Down => Key::Down,
            dq::Keycode::Left => Key::Left,
            dq::Keycode::Right => Key::Right,
            dq::Keycode::Space => Key::Space,
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
            _ => Key::Unknown,
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.threads_running.store(false, Ordering::SeqCst);
        self.device_thread.take().unwrap().join().unwrap();
        self.stdin_thread.take().unwrap().join().unwrap();
    }
}
