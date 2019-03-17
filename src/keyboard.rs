//use crossterm as ct;
//use device_query as dq;
use std::time;
use std::thread::{self, JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Key {
    Esc,
    Space,
    Enter,
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
    Unknown,
}

#[derive(Clone, Copy, Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

pub struct Keyboard {
    stdin_thread: Option<JoinHandle<()>>,
    device_thread: Option<JoinHandle<()>>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let stdin_thread = thread::spawn(move || {
            loop {
                println!("stdin_thread!");
                thread::sleep(time::Duration::from_millis(1000));
            }
        });

        let device_thread = thread::spawn(move || {
            loop {
                println!("device_thread!");
                thread::sleep(time::Duration::from_millis(1000));
            }
        });

        Keyboard {
            stdin_thread: Some(stdin_thread),
            device_thread: Some(device_thread),
        }
    }

    pub fn get_key_events() -> Vec<KeyEvent> {
        Vec::new()
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.stdin_thread.take().unwrap().join().unwrap();
        self.device_thread.take().unwrap().join().unwrap();
    }
}
