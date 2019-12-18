use crossterm as ct;
use device_query as dq;
use dq::DeviceQuery;

use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashSet;

use std::time;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Key {
    Esc, Space, Enter,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

#[derive(PartialEq, Eq, Hash)] //TODO: remove line
struct KeySignal {
    key: Key,
    stamp: time::Instant,
}

pub struct Keyboard {
    stdin_thread: Option<JoinHandle<()>>,
    device_thread: Option<JoinHandle<()>>,
    event_receiver: Receiver<KeyEvent>,
    threads_running: Arc<AtomicBool>,
    state: HashSet<KeySignal>,
    last_stamp: usize,
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
            state: HashSet::new(),
            last_stamp: 0,
        }
    }

    pub fn consume_key_events(&mut self) -> Vec<KeyEvent> {
        self.event_receiver.try_iter().collect::<Vec<_>>()
        //TODO: Check also that the release keys are previously pressed
        //TODO: Check also that the pressed keys not are previously pressed
    }

    fn process_input_event(sender: &Sender<KeyEvent>) {
        if ct::event::poll(time::Duration::from_millis(1)).unwrap() {
            let event = ct::event::read().unwrap();
            if let ct::event::Event::Key(key_event) = event {
                let key = Self::transform_input_key(&key_event.code);
                sender.send(KeyEvent::Pressed(key)).unwrap();
            }
        }
    }

    fn process_device_event(sender: &Sender<KeyEvent>, device: &dq::DeviceState, last_device_state: &mut Vec<dq::Keycode>) {
        let new_state = device.get_keys();
        let released: Vec<dq::Keycode> = last_device_state.clone().into_iter().filter(|x| !new_state.contains(x)).collect();
        std::mem::replace(last_device_state, new_state);
        for keycode in released {
            let key = Self::transform_device_key(&keycode);
            sender.send(KeyEvent::Released(key)).unwrap();
        }
        thread::sleep(time::Duration::from_millis(1));
    }

    fn transform_input_key(_input_key: &ct::event::KeyCode) -> Key {
        Key::Unknown
    }

    fn transform_device_key(_device_key: &dq::Keycode) -> Key {
        Key::Unknown
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.threads_running.store(false, Ordering::SeqCst);
        self.device_thread.take().unwrap().join().unwrap();
        self.stdin_thread.take().unwrap().join().unwrap();
    }
}
