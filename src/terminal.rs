use std::io;
use std::io::Write;
use std::io::BufWriter;
use std::panic;
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

// ================================================================================
// VISUAL ELEMENT
// ================================================================================
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

// ================================================================================
// SURFACE
// ================================================================================
pub struct Surface {
    data: Vec<VisualElement>,
    dimension: (u16, u16)
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

        let dimension = terminal::size().unwrap();
        Window {
            surface: Surface::new(dimension),
            target: BufWriter::with_capacity(dimension.0 as usize * dimension.1 as usize * 50, io::stdout()),
            ctrlc_event,
        }
    }

    pub fn open(&mut self) {
        queue!(self.target, screen::EnterAlternateScreen).unwrap();
        queue!(self.target, cursor::Hide).unwrap();
        queue!(self.target, cursor::MoveTo(0, 0)).unwrap();
        self.target.flush().unwrap();
    }

    pub fn close(&mut self) {
        queue!(self.target, cursor::Show).unwrap();
        queue!(self.target, screen::LeaveAlternateScreen).unwrap();
        self.target.flush().unwrap();
    }

    pub fn clear(&mut self) {
        queue!(self.target, cursor::MoveTo(0, 0)).unwrap();
        self.surface.fill(&VisualElement::new());
    }

    pub fn update(&mut self) {
        for element in self.surface.data().iter() {
            queue!(self.target, Output(element.value())).unwrap();
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

