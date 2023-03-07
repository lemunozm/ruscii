//! # Terminal
//!
//! The `terminal` module contains declarations that interact with the terminal. It provides
//! functionality for styling text and other characters as well as implementations for drawing
//! them through a [`Canvas`] and [`Window`].
//!
//! ## Examples
//!
//! The majority of applications do not interact with either of the [`Canvas`] or the [`Window`]
//! directly, but instead draw to the terminal using a [`Pencil`](crate::drawing::Pencil).
//!
//! An example of direct use of these elements can be seen in `window.rs` from the
//! [examples](https://github.com/lemunozm/ruscii/tree/master/examples) folder of the
//! [`ruscii`](https://github.com/lemunozm/ruscii) repository:
//!
//! ```rust,no_run
//! # use ruscii::terminal::{Window, Color, VisualElement};
//! #
//! fn main() {
//!     let mut window = Window::new();
//!     window.open();
//!     println!("This is an open window");
//!     std::thread::sleep(std::time::Duration::from_secs(2));
//!
//!     let mut default = VisualElement::new();
//!     default.background = Color::Red;
//!     window.canvas_mut().set_default_element(&default);
//!     window.clear();
//!     window.draw();
//!     println!("With a custom background color!");
//!
//!     std::thread::sleep(std::time::Duration::from_secs(2));
//!     window.close();
//! }
//! ```

use std::io::{self, Write, BufWriter};

use crossterm as ct;
use super::spatial::Vec2;

/// A set of common colors and a [`Color::Xterm`] value that allows you to pass an arbitrary ANSI
/// 8-bit color using its Xterm number (compatible with Windows 10 and most UNIX terminals).
///
/// # Example
///
/// For instance, to set a [`Pencil`](crate::drawing::Pencil)'s foreground color to Xterm color
/// DarkCyan (`#00af87`), you would do something similar to the following:
///
/// ```rust,no_run
/// # use ruscii::app::{App, State};
/// # use ruscii::drawing::Pencil;
/// # use ruscii::terminal::{Color, Window};
/// #
/// # let mut app = App::new();
/// #
/// # app.run(|app_state: &mut State, window: &mut Window| {
/// let mut pencil = Pencil::new(window.canvas_mut());
/// pencil.set_foreground(Color::Xterm(36));
/// # });
/// ```
///
/// For reference, see the
/// [256 Colors Cheat Sheet](https://www.ditig.com/256-colors-cheat-sheet).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    /// Converts this [`Color`] to its corresponding Xterm number.
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

/// The font weight.
///
/// Represents the boldness of text on the terminal screen.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Style {
    Plain,
    Bold,
}

/*
fn style_impl(style: Style) -> ct::style::Attribute {
    match style {
        Style::Plain => ct::style::Attribute::NoBold,
        Style::Bold => ct::style::Attribute::Bold,
    }
}
*/

/// Represents all the data needed to display a character on the terminal screen with text [`Style`]
/// and foreground and background [`Color`].
#[derive(Clone, Copy)]
pub struct VisualElement {
    pub style: Style,
    pub background: Color,
    pub foreground: Color,
    pub value: char,
}

impl VisualElement {
    /// Constructs a [`VisualElement`] with the default terminal styles.
    pub fn new() -> VisualElement {
        VisualElement {
            style: Style::Plain,
            background: Color::Black,
            foreground: Color::White,
            value: ' ',
        }
    }
}

/// An object that holds the data for a grid of [`VisualElements`] for a single frame.
pub struct Canvas {
    data: Vec<VisualElement>,
    dimension: Vec2,
    default_element: VisualElement,
}

impl Canvas {
    /// Constructs a [`Canvas`] with the given `dimension` and each cell set to the given
    /// [`VisualElement`] as a default.
    pub fn new(dimension: Vec2, default_element: &VisualElement) -> Canvas {
        let mut data = Vec::new();
        data.resize((dimension.x * dimension.y) as usize, *default_element);
        Canvas {
            data,
            dimension,
            default_element: *default_element,
        }
    }

    pub fn default_element(&self) -> &VisualElement {
        &self.default_element
    }

    /// Sets the current [`Canvas`]'s default element, to which every cell is reset when
    /// [`Canvas::clear`] is called.
    pub fn set_default_element(&mut self, element: &VisualElement) {
        self.default_element = *element;
    }

    pub fn dimension(&self) -> Vec2 {
        self.dimension
    }

    /// Checks if the point represented by the given `pos` would be within the dimensions of the
    /// [`Canvas`].
    ///
    /// The dimensions are sizes while the `pos` are indices.
    ///
    /// ```rust
    /// # use ruscii::spatial::Vec2;
    /// # use ruscii::terminal::{Canvas, VisualElement};
    /// #
    /// let canvas = Canvas::new(Vec2::xy(10, 20), &VisualElement::new());
    /// let a = Vec2::xy(10, 20);
    /// let b = Vec2::xy(9, 19);
    ///
    /// assert!(!canvas.contains(a));
    /// assert!(canvas.contains(b));
    /// ```
    ///
    /// A [`Vec2`] with any negative components will always evaluate `false`.
    ///
    /// ```rust
    /// # use ruscii::spatial::Vec2;
    /// # use ruscii::terminal::{Canvas, VisualElement};
    /// #
    /// let canvas = Canvas::new(Vec2::xy(10, 20), &VisualElement::new());
    /// let p = Vec2::xy(-1, -3);
    ///
    /// assert!(!canvas.contains(p));
    /// ```
    pub fn contains(&self, pos: Vec2) -> bool {
        0 <= pos.x && 0 <= pos.y &&
            pos.x < self.dimension.x &&
            pos.y < self.dimension.y
    }

    /// Returns a reference to the [`Canvas`] cell at the given `pos` if that `pos` is
    /// within the [`Canvas`] dimensions, [`None`] otherwise.
    pub fn elem(&self, pos: Vec2) -> Option<&VisualElement> {
        if self.contains(pos) {
            Some(&self.data[(pos.y * self.dimension.x + pos.x) as usize])
        } else { None }
    }

    /// Returns a mutable reference to the [`Canvas`] cell at the given `pos` if that `pos` is
    /// within the [`Canvas`] dimensions, [`None`] otherwise.
    pub fn elem_mut(&mut self, pos: Vec2) -> Option<&mut VisualElement> {
        if self.contains(pos) {
            Some(&mut self.data[(pos.y * self.dimension.x + pos.x) as usize])
        } else { None }
    }

    /// Clears all of the [`VisualElement`] cells in the grid by setting them to clones of the
    /// default element.
    pub fn clear(&mut self) {
        self.fill(&self.default_element().clone());
    }

    /// Sets every cell to the given `elem`.
    pub fn fill(&mut self, elem: &VisualElement) {
        self.data.iter_mut().map(|x| *x = *elem).count();
    }

    /// Returns a reference to all the data the [`Canvas`] holds.
    pub fn data(&self) -> &Vec<VisualElement> {
        &self.data
    }
}

/// An object that exposes a [`Canvas`] and can write the data within it to the standard output.
pub struct Window {
    canvas: Canvas,
    target: BufWriter<io::Stdout>,
}

impl Window {
    /// Constructs a [`Window`] with the automatically detected size and the target set to the
    /// [`io::stdout`].
    pub fn new() -> Window {
        Window {
            canvas: Canvas::new(size(), &VisualElement::new()),
            target: BufWriter::with_capacity(size().x as usize * size().y as usize * 50, io::stdout()),
        }
    }

    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn size(&self) -> Vec2 {
        self.canvas.dimension()
    }

    pub fn open(&mut self) {
        ct::queue!(self.target, ct::terminal::EnterAlternateScreen).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::cursor::Hide).unwrap();

        self.clean_state();
        self.raw_mode(true);

        self.target.flush().unwrap();
    }

    pub fn raw_mode(&mut self, enable: bool) {
        if enable {
            ct::terminal::enable_raw_mode().unwrap();
        } else {
            ct::terminal::disable_raw_mode().unwrap();
        }
    }

    pub fn close(&mut self) {
        self.raw_mode(false);

        ct::queue!(self.target, ct::cursor::Show).unwrap();
        ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::Reset)).unwrap();
        ct::queue!(self.target, ct::style::ResetColor).unwrap();
        ct::queue!(self.target, ct::terminal::LeaveAlternateScreen).unwrap();

        self.target.flush().unwrap();
    }

    pub fn clear(&mut self) {
        if self.canvas.dimension() != size() {
            self.canvas = Canvas::new(size(), self.canvas.default_element());
        } else {
            self.canvas.fill(&self.canvas.default_element().clone());
        }
    }

    pub fn draw(&mut self) {
        self.clean_state();
        let mut last_foreground = self.canvas.default_element().foreground;
        let mut last_background = self.canvas.default_element().background;
        //let mut last_style = self.canvas.default_element().style;
        let target = &mut self.target;

        for element in self.canvas.data().iter() {
            /*
            if last_style != element.style {
                let term_attribute = style_impl(element.style);
                ct::queue!(self.target, ct::style::SetAttribute(term_attribute)).unwrap();
                last_style = element.style
            }
            */
            if last_foreground != element.foreground {
                let term_color = ct::style::Color::AnsiValue(element.foreground.code());
                ct::queue!(target, ct::style::SetForegroundColor(term_color)).unwrap();
                last_foreground = element.foreground
            }
            if last_background != element.background {
                let term_color = ct::style::Color::AnsiValue(element.background.code());
                ct::queue!(target, ct::style::SetBackgroundColor(term_color)).unwrap();
                last_background = element.background
            }
            ct::queue!(target, ct::style::Print(element.value)).unwrap();
        }
        self.clean_state();
        self.target.flush().unwrap();
    }

    fn clean_state(&mut self) {
        //ct::queue!(self.target, ct::style::SetAttribute(ct::style::Attribute::NoBold)).unwrap();

        let term_foreground = ct::style::Color::AnsiValue(self.canvas.default_element().foreground.code());
        ct::queue!(self.target, ct::style::SetForegroundColor(term_foreground)).unwrap();

        let term_background = ct::style::Color::AnsiValue(self.canvas.default_element().background.code());
        ct::queue!(self.target, ct::style::SetBackgroundColor(term_background)).unwrap();

        ct::queue!(self.target, ct::cursor::MoveTo(0, 0)).unwrap();
    }
}

/// Returns the detected size of the terminal.
pub fn size() -> Vec2 {
    let (x, y) = ct::terminal::size().unwrap();
    Vec2::xy(x, y)
}