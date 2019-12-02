use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm as ct;

use ctrlc;

// ================================================================================
// KEYEVENT
// ================================================================================
pub use ct::input::KeyEvent;

// ================================================================================
// VISUAL ELEMENT
// ================================================================================
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
    Grey,
    DarkGrey,
    LightGrey,
    Red,
    Green,
    Blue,
    Cyan,
    Yellow,
    Magenta,
    Xterm(u8),
}

impl Color {
    pub fn code(&self) -> u8 {
        match *self {
            Color::Black => 16,
            Color::White => 231,
            Color::Grey => 244,
            Color::DarkGrey => 238,
            Color::LightGrey => 250,
            Color::Red => 196,
            Color::Green => 46,
            Color::Blue => 21,
            Color::Cyan => 51,
            Color::Yellow => 226,
            Color::Magenta => 201,
            Color::Xterm(code) => code,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Style {
    Plain,
    Bold,
}

fn style_impl(style: Style) -> ct::style::Attribute {
    match style {
        Style::Plain => ct::style::Attribute::NoBold,
        Style::Bold => ct::style::Attribute::Bold,
    }
}

#[derive(Clone, Copy)]
pub struct VisualElement {
    pub style: Style,
    pub background: Color,
    pub foreground: Color,
    pub value: char,
}

impl VisualElement {
    pub fn new() -> VisualElement {
        VisualElement {
            style: Style::Plain,
            background: Color::Black,
            foreground: Color::White,
            value: ' ',
        }
    }
}

// ================================================================================
// SURFACE
// ================================================================================
pub struct Surface {
    data: Vec<VisualElement>,
    dimension: (u16, u16),
    default_element: VisualElement,
}

impl Surface {
    pub fn new(dimension: (u16, u16), default_element: &VisualElement) -> Surface {
        let mut data = Vec::new();
        data.resize((dimension.0 * dimension.1) as usize, *default_element);
        Surface {
            data,
            dimension,
            default_element: *default_element,
        }
    }

    pub fn default_element(&self) -> &VisualElement {
        &self.default_element
    }

    pub fn set_default_element(&mut self, element: &VisualElement) {
        self.default_element = *element;
    }

    pub fn dimension(&self) -> (u16, u16) {
        self.dimension
    }

    pub fn contains(&self, pos: (u16, u16)) -> bool {
        pos.0 < self.dimension.0 && pos.1 < self.dimension.1
    }

    pub fn elem(&self, pos: (u16, u16)) -> Option<&VisualElement> {
        if self.contains(pos) {
            Some(&self.data[(pos.1 * self.dimension.0 + pos.0) as usize])
        }
        else { None }
    }

    pub fn elem_mut(&mut self, pos: (u16, u16)) -> Option<&mut VisualElement> {
        if self.contains(pos) {
            Some(&mut self.data[(pos.1 * self.dimension.0 + pos.0) as usize])
        }
        else { None }
    }

    pub fn clear(&mut self) {
        self.fill(&self.default_element().clone());
    }

    pub fn fill(&mut self, elem: &VisualElement) {
        self.data.iter_mut().map(|x| *x = *elem).count();
    }

    pub fn data(&self) -> &Vec<VisualElement> {
        &self.data
    }
}

// ================================================================================
// PENCIL
// ================================================================================
pub struct Pencil<'a> {
    origin: (u16, u16),
    foreground: Color,
    background: Color,
    style: Style,
    surface: &'a mut Surface,
}

impl<'a> Pencil<'a> {
    pub fn new(surface: &'a mut Surface) -> Pencil {
        Pencil {
            origin: (0, 0),
            foreground: surface.default_element().foreground,
            background: surface.default_element().background,
            style: surface.default_element().style,
            surface,
        }
    }

    pub fn origin(&self) -> (u16, u16) {
        self.origin
    }

    pub fn dimension(&self) -> (u16, u16) {
        (self.surface.dimension().0 - self.origin.0,
        self.surface.dimension().1 - self.origin.1)
    }

    pub fn foreground(&self) -> &Color {
        &self.foreground
    }

    pub fn background(&self) -> &Color {
        &self.background
    }

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn set_origin(mut self, pos: (u16, u16)) -> Pencil<'a> {
        self.origin = pos;
        self
    }

    pub fn set_foreground(mut self, color: Color) -> Pencil<'a> {
        self.foreground = color;
        self
    }

    pub fn set_background(mut self, color: Color) -> Pencil<'a> {
        self.background = color;
        self
    }

    pub fn set_style(mut self, style: Style) -> Pencil<'a> {
        self.style = style;
        self
    }

    pub fn draw_char(mut self, pos:(u16, u16), value: char) -> Pencil<'a> {
        let absolute = (self.origin.0 + pos.0, self.origin.1 + pos.1);
        self.draw_element(absolute, value);
        self
    }

    pub fn draw_text(mut self, pos:(u16, u16), text: &str) -> Pencil<'a> {
        for (i, value) in text.chars().enumerate() {
            let absolute = (self.origin.0 + i as u16 + pos.0, self.origin.1 + pos.1);
            self.draw_element(absolute, value);
        }
        self
    }

    fn draw_element(&mut self, pos: (u16, u16), value: char) {
        match self.surface.elem_mut(pos) {
            Some(element) => {
                element.value = value;
                element.foreground = self.foreground;
                element.background = self.background;
                element.style = self.style;
            },
            None => (),
        };
    }
}

// ================================================================================
// WINDOW
// ================================================================================
pub struct Window {
    surface: Surface,
    target: BufWriter<io::Stdout>,
    reader: Option<ct::input::AsyncReader>,
}

impl Window {
    pub fn new() -> Window {
        let dimension = ct::terminal::size().unwrap();
        Window {
            surface: Surface::new(dimension, &VisualElement::new()),
            target: BufWriter::with_capacity(dimension.0 as usize * dimension.1 as usize * 50, io::stdout()),
            reader: None,
        }
    }

    pub fn size(&self) -> (u16, u16) {
        self.surface.dimension()
    }

    pub fn open(&mut self) {
        ct::queue!(self.target, ct::screen::EnterAlternateScreen).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::cursor::Hide).unwrap();

        let mut raw = ct::screen::RawScreen::into_raw_mode().unwrap();
        raw.keep_raw_mode_on_drop();

        self.reader = Some(ct::input::input().read_async());
        self.target.flush().unwrap();
    }

    pub fn close(&mut self) {
        ct::screen::RawScreen::disable_raw_mode().unwrap();
        ct::queue!(self.target, ct::cursor::Show).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::screen::LeaveAlternateScreen).unwrap();

        match self.reader {
            Some(ref mut reader) => {
                reader.stop();
                self.reader = None;
            }
            None => panic!("You can not close a windows that is already closed"),
        }
        self.target.flush().unwrap();
    }

    pub fn clear(&mut self) {
        let current_size = ct::terminal::size().unwrap();
        if current_size.0 != self.size().0 || current_size.1 != self.size().1 {
            self.surface = Surface::new(current_size, self.surface.default_element());
        }
        else {
            self.surface.fill(&self.surface.default_element().clone());
        }
    }

    pub fn update(&mut self) {
        self.clean_state();
        let mut last_foreground = self.surface.default_element().foreground;
        let mut last_background = self.surface.default_element().background;
        let mut last_style = self.surface.default_element().style;

        for element in self.surface.data().iter() {
            if last_style != element.style {
                let term_attribute = style_impl(element.style);
                ct::queue!(self.target, ct::style::SetAttribute(term_attribute)).unwrap();
                last_style = element.style
            }
            if last_foreground != element.foreground {
                let term_color = ct::style::Color::AnsiValue(element.foreground.code());
                ct::queue!(self.target, ct::style::SetForegroundColor(term_color)).unwrap();
                last_foreground = element.foreground
            }
            if last_background != element.background {
                let term_color = ct::style::Color::AnsiValue(element.background.code());
                ct::queue!(self.target, ct::style::SetBackgroundColor(term_color)).unwrap();
                last_background = element.background
            }
            ct::queue!(self.target, ct::Output(element.value)).unwrap();
        }
        self.clean_state();
        self.target.flush().unwrap();
    }

    pub fn key_events(&mut self) -> Vec<KeyEvent> {
        let mut key_events = Vec::new();
        match self.reader {
            Some(ref mut reader) => {
                for event in reader {
                    match event {
                        ct::input::InputEvent::Keyboard(key_event) => key_events.push(key_event),
                        _ => key_events.push(KeyEvent::Esc),
                    }
                }
            }
            None => panic!("It is necessary to open the window before read events"),
        }
        key_events
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    fn clean_state(&mut self) {
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::NoBold)).unwrap();

        let term_foreground = ct::style::Color::AnsiValue(self.surface.default_element().foreground.code());
        ct::queue!(self.target, ct::style::SetForegroundColor(term_foreground)).unwrap();

        let term_background = ct::style::Color::AnsiValue(self.surface.default_element().background.code());
        ct::queue!(self.target, ct::style::SetBackgroundColor(term_background)).unwrap();

        ct::queue!(self.target, ct::cursor::MoveTo(0, 0)).unwrap();
    }
}

// ================================================================================
// CONFIG and STATE
// ================================================================================
pub struct Config {
    pub fps: u32,
    pub ctrl_c: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {fps: 60, ctrl_c: true}
    }

    pub fn fps(mut self, fps: u32) -> Config {
        self.fps = fps;
        self
    }

    pub fn ctrl_c(mut self, value: bool) -> Config {
        self.ctrl_c = value;
        self
    }
}

pub struct State {
    pub abort: bool,
}

impl State {
    pub fn new() -> State {
        State {abort: false}
    }
}

// ================================================================================
// MODULE
// ================================================================================
pub fn run<F>(config: Config, mut frame_action: F)
where F: FnMut(&mut State, &mut Window) {
    let expected_duration = time::Duration::from_nanos(1_000_000_000 / config.fps as u64);
    let ctrlc_event = Arc::new(AtomicBool::new(false));
    let ctrlc_event_write = ctrlc_event.clone();
    ctrlc::set_handler(move || {
        ctrlc_event_write.store(true, Ordering::SeqCst);
    }).unwrap();

    let mut window = Window::new();
    let mut state = State::new();
    window.open();
    loop {
        let now = time::Instant::now();
        window.clear();

        state.abort = config.ctrl_c && ctrlc_event.load(Ordering::SeqCst);
        frame_action(&mut state, &mut window);
        if state.abort {
            break;
        }

        window.update();

        if let Some(time) = expected_duration.checked_sub(now.elapsed()) {
            thread::sleep(time);
        }
    }
    window.close();
}

