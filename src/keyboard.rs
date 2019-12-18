use crossterm as ct;
//use device_query as dq;

use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)] //TODO: remove line
pub struct KeySignal { //TODO: remove pub
    event: KeyEvent,
    stamp: time::Instant,
}

pub struct Keyboard {
    stdin_thread: Option<JoinHandle<()>>,
    device_thread: Option<JoinHandle<()>>,
    event_receiver: Receiver<KeySignal>,
    threads_running: Arc<AtomicBool>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let threads_running = Arc::new(AtomicBool::new(true));
        let (sender, receiver): (Sender<KeySignal>, Receiver<KeySignal>) = mpsc::channel();

        let pressed_sender = sender.clone();
        let stdin_running = threads_running.clone();
        let stdin_thread = thread::spawn(move || {
            while stdin_running.load(Ordering::SeqCst) {
                if ct::event::poll(time::Duration::from_millis(1)).unwrap() {
                    match ct::event::read().unwrap() {
                        ct::event::Event::Key(key_event) => {
                            match key_event.code {
                                ct::event::KeyCode::Char('c') =>
                                    pressed_sender.send(KeySignal{event: KeyEvent::Pressed(Key::C), stamp: time::Instant::now()}).unwrap(),
                                _ => (),
                            }
                        },
                        _ => ()
                    }
                }
            }
        });

        let released_sender = sender.clone();
        let device_running = threads_running.clone();
        let device_thread = thread::spawn(move || {
            while device_running.load(Ordering::SeqCst) {
                thread::sleep(time::Duration::from_millis(1));
                released_sender.send(KeySignal{event: KeyEvent::Released(Key::A), stamp: time::Instant::now()}).unwrap();
            }
        });

        Keyboard {
            stdin_thread: Some(stdin_thread),
            device_thread: Some(device_thread),
            event_receiver: receiver,
            threads_running
        }
    }

    pub fn get_key_events(&mut self) -> Vec<KeyEvent> {
        Vec::new()
    }

    pub fn get_signals(&self) -> Vec<KeySignal> {
        self.event_receiver.try_iter().collect::<Vec<_>>()
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.threads_running.store(false, Ordering::SeqCst);
        self.device_thread.take().unwrap().join().unwrap();
        self.stdin_thread.take().unwrap().join().unwrap();
    }
}
