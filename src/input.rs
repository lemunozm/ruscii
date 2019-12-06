use crossterm as ct;
use device_query as dq;
use dq::DeviceQuery;

// ================================================================================
// KEYDOWN
// ================================================================================
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

fn to_local_key(key_code: dq::Keycode) -> Key {
    match key_code {
        dq::Keycode::Escape => Key::Esc,
        dq::Keycode::Space => Key::Space,
        dq::Keycode::Enter => Key::Enter,
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

#[derive(Clone, Copy, Debug)]
pub enum KeyDown {
    Key(Key),
    Ctrl(Key),
    Alt(Key),
    Shift(Key),
    CtrlAlt(Key),
    CtrlShift(Key),
    AltShift(Key),
    CtrlAltShift(Key),
}

const NONE: usize = 0x00;
const CTRL: usize = 0x01;
const ALT: usize = 0x02;
const SHIFT: usize = 0x04;
const CTRL_ALT: usize = CTRL | ALT;
const CTRL_SHIFT: usize = CTRL | SHIFT;
const ALT_SHIFT: usize = ALT | SHIFT;
const CTRL_ALT_SHIFT: usize = CTRL | ALT | SHIFT;

pub fn get_keys_down() -> Vec<KeyDown> {
    let mut modifier = NONE;
    let mut key_list = Vec::new();
    for key_down in (*DEVICE).device.get_keys() {
        match key_down {
            dq::Keycode::LControl | dq::Keycode::RControl => modifier |= CTRL,
            dq::Keycode::LAlt | dq::Keycode::RAlt => modifier |= ALT,
            dq::Keycode::LShift | dq::Keycode::RShift => modifier |= SHIFT,
            other => key_list.push(to_local_key(other)),
        }
    }

    match modifier {
        NONE => key_list.iter().map(|key| KeyDown::Key(*key)).collect::<Vec<KeyDown>>(),
        CTRL => key_list.iter().map(|key| KeyDown::Ctrl(*key)).collect::<Vec<KeyDown>>(),
        ALT => key_list.iter().map(|key| KeyDown::Alt(*key)).collect::<Vec<KeyDown>>(),
        SHIFT => key_list.iter().map(|key| KeyDown::Shift(*key)).collect::<Vec<KeyDown>>(),
        CTRL_ALT => key_list.iter().map(|key| KeyDown::CtrlAlt(*key)).collect::<Vec<KeyDown>>(),
        CTRL_SHIFT => key_list.iter().map(|key| KeyDown::CtrlShift(*key)).collect::<Vec<KeyDown>>(),
        ALT_SHIFT => key_list.iter().map(|key| KeyDown::AltShift(*key)).collect::<Vec<KeyDown>>(),
        CTRL_ALT_SHIFT=> key_list.iter().map(|key| KeyDown::CtrlAltShift(*key)).collect::<Vec<KeyDown>>(),
        _ => unreachable!(),
    }
}

struct SyncDevice {
   device: dq::DeviceState
}

impl SyncDevice {
    pub fn new() -> SyncDevice {
        SyncDevice { device: dq::DeviceState::new() }
    }
}

unsafe impl Sync for SyncDevice { }

lazy_static! {
    static ref DEVICE: SyncDevice = SyncDevice::new();
}

// ================================================================================
// KEYEVENT
// ================================================================================
pub use ct::input::KeyEvent;

pub struct KeyReader {
   input_reader: Option<ct::input::AsyncReader>,
}

impl KeyReader {
    pub fn new() -> KeyReader {
        KeyReader { input_reader: None }
    }

    pub fn start(&mut self) {
        self.input_reader = Some(ct::input::input().read_async());
    }

    pub fn stop(&mut self) {
        if let Some(ref mut reader) = self.input_reader {
            reader.stop();
            self.input_reader = None;
        }
    }

    pub fn key_events(&mut self) -> Vec<KeyEvent> {
        let mut key_events = Vec::new();
        match self.input_reader {
            Some(ref mut reader) => {
                for event in reader {
                    match event {
                        ct::input::InputEvent::Keyboard(key_event) => key_events.push(key_event),
                        _ => (),
                    }
                }
            }
            None => panic!("It is necessary to start the reader before read key events"),
        }
        key_events
    }
}

