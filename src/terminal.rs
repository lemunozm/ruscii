use std::io;
use std::io::Write;
use std::collections::hash_set::HashSet;

use crossterm::{
    queue,
    screen,
    terminal::{Clear, ClearType},
};

#[derive(Clone, Eq, PartialEq, Hash)]
enum Style {
    Plain,
    Bold,
    Italic,
}

#[derive(Clone)]
pub struct VisualElement {
    styles: HashSet<Style>,
    background: u8,
    foreground: u8,
    value: char,
}

pub struct Surface {
    data: Vec<VisualElement>,
    dimension: (u32, u32)
}

pub struct Pencil<'a> {
    surface: &'a mut Surface,
    position: (u32, u32),
}

pub struct Window {
    surface: Surface,
    target: io::Stdout,
}

impl VisualElement {
    pub fn new() -> VisualElement {
        VisualElement {
            styles: HashSet::new(),
            background: 0,
            foreground: 0,
            value: ' ',
        }
    }
}

impl Surface {
    pub fn new(width: u32, height: u32) -> Surface {
        let mut data = Vec::new();
        data.resize((width * height) as usize, VisualElement::new());
        Surface {
            data,
            dimension: (width, height),
        }
    }

    pub fn elem(&self, pos: (u32, u32)) -> &VisualElement {
        &self.data[(pos.0 * self.dimension.0 + pos.1) as usize]
    }

    pub fn elem_mut(&mut self, pos: (u32, u32)) -> &mut VisualElement {
        &mut self.data[(pos.0 * self.dimension.0 + pos.1) as usize]
    }

    pub fn fill(&mut self, value: &VisualElement) {
        self.data.iter_mut().map(|_| value).count();
    }
}

impl<'a> Pencil<'a> {
    pub fn new(surface: &'a mut Surface) -> Pencil {
        Pencil {
            surface,
            position: (0, 0)
        }
    }

    pub fn move_to(&mut self, pos: (u32, u32)) {
        self.position = pos;
    }

    pub fn draw(&mut self, value: char) {
        self.surface.elem_mut(self.position).value = value
    }
}


impl Window {
    pub fn open() -> Window {
        let mut target = io::stdout();
        queue!(target, screen::EnterAlternateScreen).unwrap();
        Window {
            surface: Surface::new(10, 10),
            target: io::stdout(),
        }
    }

    pub fn close(&mut self) {
        queue!(self.target, screen::LeaveAlternateScreen).unwrap();
    }

    pub fn clear(&mut self) {
        self.surface.fill(&VisualElement::new());
        queue!(self.target, Clear(ClearType::All)).unwrap();
    }

    pub fn update(&mut self) {
        //TODO
    }

    pub fn surface(&self) -> &Surface { &self.surface }
    pub fn surface_mut(&mut self) -> &mut Surface { &mut self.surface }
}

