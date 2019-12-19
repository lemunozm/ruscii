use std::io::{self, Write, BufWriter};

use crossterm as ct;

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
pub struct Canvas {
    data: Vec<VisualElement>,
    dimension: (u16, u16),
    default_element: VisualElement,
}

impl Canvas {
    pub fn new(dimension: (u16, u16), default_element: &VisualElement) -> Canvas {
        let mut data = Vec::new();
        data.resize((dimension.0 * dimension.1) as usize, *default_element);
        Canvas {
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
    canvas: &'a mut Canvas,
}

impl<'a> Pencil<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Pencil {
        Pencil {
            origin: (0, 0),
            foreground: canvas.default_element().foreground,
            background: canvas.default_element().background,
            style: canvas.default_element().style,
            canvas,
        }
    }

    pub fn origin(&self) -> (u16, u16) {
        self.origin
    }

    pub fn dimension(&self) -> (u16, u16) {
        (self.canvas.dimension().0 - self.origin.0,
        self.canvas.dimension().1 - self.origin.1)
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

    pub fn set_origin(&mut self, pos: (u16, u16)) -> &mut Pencil<'a> {
        self.origin = pos;
        self
    }

    pub fn set_foreground(&mut self, color: Color) -> &mut Pencil<'a> {
        self.foreground = color;
        self
    }

    pub fn set_background(&mut self, color: Color) -> &mut Pencil<'a> {
        self.background = color;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Pencil<'a> {
        self.style = style;
        self
    }

    pub fn draw_char(&mut self, value: char, pos:(u16, u16)) -> &mut Pencil<'a> {
        let elem_pos = (self.origin.0 + pos.0, self.origin.1 + pos.1);
        self.draw_element(elem_pos, value);
        self
    }

    pub fn draw_text(&mut self, text: &str, pos:(u16, u16)) -> &mut Pencil<'a> {
        let width = self.canvas.dimension().0;
        for (i, value) in text.chars().enumerate() {
            let elem_pos = (self.origin.0 + i as u16 + pos.0, self.origin.1 + pos.1);
            let elem_pos = (elem_pos.0 % width, elem_pos.1 + elem_pos.0 / width);
            self.draw_element(elem_pos, value);
        }
        self
    }

    pub fn draw_vline(&mut self, value: char, from:(u16, u16), size: u16) -> &mut Pencil<'a> {
        let elem_pos = (self.origin.0 + from.0, self.origin.1 + from.1);
        for i in 0..size {
            let position = (elem_pos.0, elem_pos.1 + i);
            self.draw_element(position, value);
        }
        self
    }

    pub fn draw_hline(&mut self, value: char, from:(u16, u16), size: u16) -> &mut Pencil<'a> {
        let elem_pos = (self.origin.0 + from.0, self.origin.1 + from.1);
        for i in 0..size {
            let position = (elem_pos.0 + i, elem_pos.1);
            self.draw_element(position, value);
        }
        self
    }

    fn draw_element(&mut self, pos: (u16, u16), value: char) {
        match self.canvas.elem_mut(pos) {
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
    canvas: Canvas,
    target: BufWriter<io::Stdout>,
}

impl Window {
    pub fn new() -> Window {
        Window {
            canvas: Canvas::new(size(), &VisualElement::new()),
            target: BufWriter::with_capacity(size().0 as usize * size().1 as usize * 50, io::stdout()),
        }
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn size(&self) -> (u16, u16) {
        self.canvas.dimension()
    }

    pub fn open(&mut self) {
        ct::queue!(self.target, ct::terminal::EnterAlternateScreen).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::cursor::Hide).unwrap();

        ct::terminal::enable_raw_mode().unwrap();

        self.target.flush().unwrap();
    }

    pub fn close(&mut self) {
        ct::terminal::disable_raw_mode().unwrap();

        ct::queue!(self.target, ct::cursor::Show).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::terminal::LeaveAlternateScreen).unwrap();

        self.target.flush().unwrap();
    }

    pub fn clear(&mut self) {
        if size().0 != self.canvas.dimension().0 || size().1 != self.canvas.dimension().1 {
            self.canvas = Canvas::new(size(), self.canvas.default_element());
        }
        else {
            self.canvas.fill(&self.canvas.default_element().clone());
        }
    }

    pub fn update(&mut self) {
        self.clean_state();
        let mut last_foreground = self.canvas.default_element().foreground;
        let mut last_background = self.canvas.default_element().background;
        let mut last_style = self.canvas.default_element().style;

        for element in self.canvas.data().iter() {
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
            ct::queue!(self.target, ct::style::Print(element.value)).unwrap();
        }
        self.clean_state();
        self.target.flush().unwrap();
    }

    fn clean_state(&mut self) {
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::NoBold)).unwrap();

        let term_foreground = ct::style::Color::AnsiValue(self.canvas.default_element().foreground.code());
        ct::queue!(self.target, ct::style::SetForegroundColor(term_foreground)).unwrap();

        let term_background = ct::style::Color::AnsiValue(self.canvas.default_element().background.code());
        ct::queue!(self.target, ct::style::SetBackgroundColor(term_background)).unwrap();

        ct::queue!(self.target, ct::cursor::MoveTo(0, 0)).unwrap();
    }
}

pub fn size() -> (u16, u16) {
    ct::terminal::size().unwrap()
}

