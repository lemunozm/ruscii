use std::io;
use std::panic;
use std::panic::RefUnwindSafe;
use std::io::Write;
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crossterm::{
    queue,
    screen,
    terminal,
    cursor,
    Output,
};

use ctrlc;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Style {
    None,
    Plain,
    Bold,
    Italic,
}

#[derive(Clone, Copy)]
pub struct VisualElement {
    pub style: Style,
    pub background: u8,
    pub foreground: u8,
    pub value: char,
}

pub struct Surface {
    data: Vec<VisualElement>,
    dimension: (u16, u16)
}

pub struct Pencil<'a> {
    surface: &'a mut Surface,
    origin: (u16, u16),
    dimension: (u16, u16),
}

pub struct Window {
    surface: Surface,
    target: io::Stdout,
    ctrlc_event: Arc<AtomicBool>,
}

impl VisualElement {
    pub fn new() -> VisualElement {
        VisualElement {
            style: Style::None,
            background: 0,
            foreground: 0,
            value: ' ',
        }
    }

    pub fn value(&self) -> char {
        self.value
    }
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
        else {
            None
        }
    }

    pub fn elem_mut(&mut self, pos: (u16, u16)) -> Option<&mut VisualElement> {
        if self.contains(pos) {
            Some(&mut self.data[(pos.1 * self.dimension.0 + pos.0) as usize])
        }
        else {
            None
        }
    }

    pub fn fill(&mut self, elem: &VisualElement) {
        self.data.iter_mut().map(|x| *x = *elem).count();
    }
}

impl<'a> Pencil<'a> {
    pub fn new(surface: &'a mut Surface) -> Pencil {
        let dimension = surface.dimension();
        Pencil {
            surface,
            origin: (0, 0),
            dimension,
        }
    }

    pub fn origin(&self) -> (u16, u16) {
        self.origin
    }

    pub fn dimension(&self) -> (u16, u16) {
        self.dimension
    }

    pub fn set_origin(&mut self, pos: (u16, u16)) {
        self.origin = pos;
    }

    pub fn draw_char(&mut self, pos:(u16, u16), value: char) {
        let position = (self.origin.0 + pos.0, self.origin.1 + pos.1);
        match self.surface.elem_mut(position) {
            Some(element) => element.value = value,
            None => panic!("Out of surface"),
        }
    }

    pub fn draw_text(&mut self, pos:(u16, u16), text: &str) {
        for (i, c) in text.chars().enumerate() {
            let position = (self.origin.0 + i as u16 + pos.0, self.origin.1 + pos.1);
            match self.surface.elem_mut(position) {
                Some(element) => element.value = c,
                None => panic!("Out of surface"),
            }
        }
    }
}

impl Window {
    pub fn new() -> Window {
        let ctrlc_event = Arc::new(AtomicBool::new(false));
        let ctrlc_event_write = ctrlc_event.clone();
        ctrlc::set_handler(move || {
            ctrlc_event_write.store(true, Ordering::SeqCst);
        }).unwrap();

        let dimension = terminal::size().unwrap();
        Window {
            surface: Surface::new(dimension),
            target: io::stdout(),
            ctrlc_event,
        }
    }

    pub fn open(&mut self) {
        queue!(self.target, screen::EnterAlternateScreen).unwrap();
        queue!(self.target, cursor::Hide).unwrap();
        queue!(self.target, cursor::MoveTo(0, 0)).unwrap();
    }

    pub fn close(&mut self) {
        queue!(self.target, cursor::Show).unwrap();
        queue!(self.target, screen::LeaveAlternateScreen).unwrap();
    }

    pub fn clear(&mut self) {
        queue!(self.target, cursor::MoveTo(0, 0)).unwrap();
        self.surface.fill(&VisualElement::new());
    }

    pub fn update(&mut self) {
        let (width, height) = self.surface.dimension();
        for y in 1..height {
            for x in 1..width {
                queue!(self.target, cursor::MoveTo(x, y)).unwrap();
                queue!(self.target, Output(self.surface.elem((x, y)).unwrap().value())).unwrap();
            }
        }
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
}

pub fn run<F>(fps: u32, mut frame_action: F)
where F: FnMut(&mut Window) -> bool + RefUnwindSafe {
    let mut window = Window::new();
    window.open();
    loop {
        window.clear();
        if window.was_aborted() {
            break;
        }
        if !frame_action(&mut window) {
            break;
        }
        window.update();

        thread::sleep(time::Duration::from_micros((1.0 / fps as f32) as u64));
    }
    window.close();
}

