use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::panic;
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm as ct;

use ctrlc;

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
}

impl Surface {
    pub fn new(dimension: (u16, u16)) -> Surface {
        let mut data = Vec::new();
        data.resize((dimension.0 * dimension.1) as usize, VisualElement::new());
        Surface {
            data,
            dimension,
        }
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
    surface: &'a mut Surface,
    origin: (u16, u16),
    dimension: (u16, u16),
    foreground: Color,
    background: Color,
    style: Style,
}

impl<'a> Pencil<'a> {
    pub fn new(surface: &'a mut Surface) -> Pencil {
        let dimension = surface.dimension();
        Pencil {
            surface,
            origin: (0, 0),
            dimension,
            foreground: VisualElement::new().foreground,
            background: VisualElement::new().background,
            style: VisualElement::new().style,
        }
    }

    pub fn origin(&self) -> (u16, u16) {
        self.origin
    }

    pub fn dimension(&self) -> (u16, u16) {
        self.dimension
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
            None => panic!("Out of surface"),
        };
    }
}

// ================================================================================
// WINDOW
// ================================================================================
pub struct Window {
    surface: Surface,
    target: BufWriter<io::Stdout>,
    ctrlc_event: Arc<AtomicBool>,
}

impl Window {
    pub fn new() -> Window {
        let ctrlc_event = Arc::new(AtomicBool::new(false));
        let ctrlc_event_write = ctrlc_event.clone();
        ctrlc::set_handler(move || {
            ctrlc_event_write.store(true, Ordering::SeqCst);
        }).unwrap();

        let dimension = ct::terminal::size().unwrap();
        Window {
            surface: Surface::new(dimension),
            target: BufWriter::with_capacity(dimension.0 as usize * dimension.1 as usize * 50, io::stdout()),
            ctrlc_event,
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
        self.target.flush().unwrap();
    }

    pub fn close(&mut self) {
        ct::queue!(self.target, ct::cursor::Show).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::screen::LeaveAlternateScreen).unwrap();
        self.target.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.surface.fill(&VisualElement::new());
    }

    pub fn clear_with(&mut self, element: &VisualElement) {
        self.surface.fill(element);
    }

    pub fn update(&mut self) {
        self.clean_state();
        let mut last_foreground = VisualElement::new().foreground;
        let mut last_background = VisualElement::new().background;
        let mut last_style = VisualElement::new().style;

        for element in self.surface.data().iter() {
            if last_style != element.style {
                let term_attribute = self.to_attribute(element.style);
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

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    pub fn was_aborted(&self) -> bool {
        self.ctrlc_event.load(Ordering::SeqCst)
    }

    fn clean_state(&mut self) {
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::NoBold)).unwrap();

        let term_foreground = ct::style::Color::AnsiValue(VisualElement::new().foreground.code());
        ct::queue!(self.target, ct::style::SetForegroundColor(term_foreground)).unwrap();

        let term_background = ct::style::Color::AnsiValue(VisualElement::new().background.code());
        ct::queue!(self.target, ct::style::SetBackgroundColor(term_background)).unwrap();

        ct::queue!(self.target, ct::cursor::MoveTo(0, 0)).unwrap();
    }

    fn to_attribute(&self, style: Style) -> ct::style::Attribute {
        match style {
            Style::Plain => ct::style::Attribute::NoBold,
            Style::Bold => ct::style::Attribute::Bold,
        }
    }
}

// ================================================================================
// MODULE
// ================================================================================
pub fn run<F>(fps: u32, mut frame_action: F)
where F: FnMut(&mut Window) -> bool {
    let expected_duration = time::Duration::from_nanos(1_000_000_000 / fps as u64);
    let mut window = Window::new();
    window.open();
    loop {
        let now = time::Instant::now();
        window.clear();
        if window.was_aborted() {
            break;
        }
        if !frame_action(&mut window) {
            break;
        }
        window.update();

        if let Some(time) = expected_duration.checked_sub(now.elapsed()) {
            thread::sleep(time);
        }
    }
    window.close();
}

