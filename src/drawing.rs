//! # Drawing
//!
//! The `drawing` module provides functionality for drawing shapes and text with multiple styles
//! to the terminal screen.

use super::terminal::{Canvas, Color, Style};
use super::spatial::{Vec2};

use num::cast::ToPrimitive;

/// The set of all characters needed to draw all edges and corners of a variable-length rectangle
/// in the terminal.
#[derive(Debug, Clone)]
pub struct RectCharset {
    pub top: char,
    pub bottom: char,
    pub left: char,
    pub right: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

impl RectCharset {
    /// Returns a [RectCharset] for a single-line rectangle.
    ///
    /// # Rectangle
    ///
    /// Using this charset will provide a rectangle that looks like this:
    ///
    /// ```
    /// ┌──────┐
    /// │ruscii│
    /// └──────┘
    /// ```
    pub fn simple_lines() -> RectCharset {
        RectCharset::from("──││┌┐└┘")
    }

    /// Returns a [RectCharset] for a single-line rounded-corner rectangle.
    ///
    /// # Rectangle
    ///
    /// Using this charset will provide a rectangle that looks like this:
    ///
    /// ```
    /// ╭──────╮
    /// │ruscii│
    /// ╰──────╯
    /// ```
    pub fn simple_round_lines() -> RectCharset {
        RectCharset::from("──││╭╮╰╯")
    }

    /// Returns a [RectCharset] for a double-line rectangle.
    ///
    /// # Rectangle
    ///
    /// Using this charset will provide a rectangle that looks like this:
    ///
    /// ```
    /// ╔══════╗
    /// ║ruscii║
    /// ╚══════╝
    /// ```
    pub fn double_lines() -> RectCharset {
        RectCharset::from("══║║╔╗╚╝")
    }
}

impl From<&str> for RectCharset {
    /// A utility function that generates a [RectCharset] from the characters in a `&str`.
    ///
    /// # Panics
    ///
    /// This function will `panic!` if the given `item` is a `&str` of fewer than 8 characters.
    fn from(item: &str) -> Self {
        if item.len() < 8 {
            panic!("Build a RectCharset requires at least 8 characters.");
        }

        let mut chars = item.chars();
        RectCharset {
            top: chars.next().unwrap(),
            bottom: chars.next().unwrap(),
            left: chars.next().unwrap(),
            right: chars.next().unwrap(),
            top_left: chars.next().unwrap(),
            top_right: chars.next().unwrap(),
            bottom_left: chars.next().unwrap(),
            bottom_right: chars.next().unwrap(),
        }
    }
}

/// An interface for types that can be drawn by a [Pencil].
pub trait Drawable {
    fn draw(&self, pencil: Pencil);
}

/// An object that stores several text style options and the [Canvas] to which text and shapes can
/// be written.
///
/// # Options
///
/// - Origin - A [Vec2] on the [Canvas] which becomes the new "origin" for drawn characters. All
///   [Vec2]s subsequently passed to associated functions are relative to this [Vec2]. For example,
///   if the origin is set to (2, 3) and [draw_text()](Pencil::draw_text) is called with position
///   (1, 0), the string will be drawn at (3, 3).
/// - Foreground (character) [Color]
/// - Background [Color]
/// - [Style] (boldness)
///
/// # Examples
///
/// An introductory example using a [Pencil] could be:
///
/// ```rust,no_run
/// # use ruscii::app::{App, State};
/// # use ruscii::terminal::{Window};
/// # use ruscii::drawing::{Pencil};
/// # use ruscii::spatial::{Vec2};
/// #
/// # fn main() {
/// #    let mut app = App::new();
/// #
/// #    app.run(|app_state: &mut State, window: &mut Window| {
/// let mut pencil = Pencil::new(window.canvas_mut());
/// pencil.draw_text("Hello, world!", Vec2::xy(0, 0));
/// #    });
/// # }
/// ```
///
/// Most associated functions return a mutable reference to the function's receiver (`self`),
/// allowing for chaining multiple calls as in the following example.
///
/// From `pong.rs`:
///
/// ```rust,ignore
/// Pencil::new(window.canvas_mut())
///     .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0))
///     .set_origin(Vec2::xy((win_size.x - score_msg.len() as i32) / 2, (win_size.y - state.dimension.y) / 2 - 1))
///     .draw_text(score_msg, Vec2::xy(0, 0))
///     .set_origin((win_size - state.dimension) / 2)
///     .draw_rect(&RectCharset::simple_round_lines(), Vec2::zero(), state.dimension)
///     .draw_vline('\'', Vec2::xy(state.dimension.x / 2, 1), state.dimension.y - 2)
///     .set_foreground(Color::Blue)
///     .draw_rect(&RectCharset::double_lines(), state.left_player.position - Vec2::y(PAD_HEIGHT), Vec2::xy(2, PAD_HEIGHT * 2))
///     .set_foreground(Color::Red)
///     .draw_rect(&RectCharset::double_lines(), state.right_player.position - Vec2::y(PAD_HEIGHT), Vec2::xy(2, PAD_HEIGHT * 2))
///     .set_foreground(Color::Yellow)
///     .set_style(Style::Bold)
///     .draw_char('o', state.ball_position);
///```
pub struct Pencil<'a> {
    origin: Vec2,
    foreground: Color,
    background: Color,
    style: Style,
    canvas: &'a mut Canvas,
}

impl<'a> Pencil<'a> {
    /// Constructs a [Pencil] that can write to the given [Canvas].
    pub fn new(canvas: &'a mut Canvas) -> Pencil {
        Pencil {
            origin: Vec2::zero(),
            foreground: canvas.default_element().foreground,
            background: canvas.default_element().background,
            style: canvas.default_element().style,
            canvas,
        }
    }

    pub fn new_one(&mut self) -> Pencil {
        Pencil {
            origin: self.origin,
            foreground: self.foreground,
            background: self.background,
            style: self.style,
            canvas: self.canvas,
        }
    }

    fn draw_element(&mut self, position: Vec2, value: char) {
        match self.canvas.elem_mut(position) {
            Some(element) => {
                element.value = value;
                element.foreground = self.foreground;
                element.background = self.background;
                element.style = self.style;
            }
            None => (),
        };
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn dimension(&self) -> Vec2 {
        self.canvas.dimension() - self.origin
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

    pub fn set_origin(&mut self, position: Vec2) -> &mut Pencil<'a> {
        self.origin = position;
        self
    }

    /// Moves the origin by the given `displacement`.
    pub fn move_origin(&mut self, displacement: Vec2) -> &mut Pencil<'a> {
        self.origin += displacement;
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

    /// Draws a character at the given `position` according to the previously set text style options.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_char(&mut self, value: char, position: Vec2) -> &mut Pencil<'a> {
        self.draw_element(self.origin + position, value);
        self
    }

    /// Draws a string at the given `position` according to the previously set text style options.
    ///
    /// If the string has multiple characters, each subsequent character is drawn one point to the
    /// right (as you'd probably expect).
    ///
    /// Returns the receiver for chaining.
    pub fn draw_text(&mut self, text: &str, position: Vec2) -> &mut Pencil<'a> {
        let width = self.canvas.dimension().x;
        for (i, value) in text.chars().enumerate() {
            let elem_pos = self.origin + position + Vec2::x(i);
            let elem_pos = Vec2::xy(elem_pos.x % width, elem_pos.y + elem_pos.x / width);
            self.draw_element(elem_pos, value);
        }
        self
    }

    /// Draws a string centered at the given `position` according to the previously set text style
    /// options.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_center_text(&mut self, text: &str, position: Vec2) -> &mut Pencil<'a> {
        let position = position - Vec2::x(text.len() as i32 / 2);
        self.draw_text(text, position)
    }

    /// Draws a vertical line starting from the given `position` and extending for `size`
    /// lines downwards. This line is composed of the given `value` characters.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_vline<T: ToPrimitive>(&mut self, value: char, position: Vec2, size: T) -> &mut Pencil<'a> {
        let elem_pos = self.origin + position;
        for i in 0..size.to_usize().unwrap() as usize {
            self.draw_element(elem_pos + Vec2::y(i), value);
        }
        self
    }

    /// Draws a horizontal line starting from the given `position` and extending for `size`
    /// lines rightwards. This line is composed of the given `value` characters.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_hline<T: ToPrimitive>(&mut self, value: char, position: Vec2, size: T) -> &mut Pencil<'a> {
        let elem_pos = self.origin + position;
        for i in 0..size.to_usize().unwrap() as usize {
            self.draw_element(elem_pos + Vec2::x(i), value);
        }
        self
    }

    /// Draws an empty rectangle from the given `charset` with the given `dimension`. The given
    /// `position` sets the position of the top-left corner of the rectangle.
    ///
    /// The given `dimension` includes the characters used to draw the rectangle, so the dimension
    /// of the enclosed space has a width of `dimension.x - 2` and a height of `dimension.y - 2`.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_rect(&mut self, charset: &RectCharset, position: Vec2, dimension: Vec2) -> &mut Pencil<'a> {
        self.move_origin(position)
            .draw_hline(charset.top, Vec2::x(0), dimension.x - 1)
            .draw_hline(charset.bottom, Vec2::xy(0, dimension.y - 1), dimension.x - 1)
            .draw_vline(charset.left, Vec2::y(0), dimension.y - 1)
            .draw_vline(charset.right, Vec2::xy(dimension.x - 1, 0), dimension.y - 1)
            .draw_char(charset.top_left, Vec2::xy(0, 0))
            .draw_char(charset.top_right, Vec2::x(dimension.x - 1))
            .draw_char(charset.bottom_left, Vec2::y(dimension.y - 1))
            .draw_char(charset.bottom_right, dimension - Vec2::xy(1, 1))
            .move_origin(-position)
    }

    /// Draws a filled rectangle from the given `charset` with the given `dimension`. The given
    /// `position` sets the position of the top-left corner of the rectangle. The rectangle is
    /// composed of the given `fill` characters.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_filled_rect(&mut self, fill: char, position: Vec2, dimension: Vec2) -> &mut Pencil<'a> {
        self.move_origin(position);
        for i in 0..dimension.x {
            self.draw_vline(fill, Vec2::xy(position.x + i, position.y), dimension.y);
        }
        self.move_origin(-position)
    }

    /// Draws a [Drawable] at and sets the origin to the given `position`.
    ///
    /// Returns the receiver for chaining.
    pub fn draw_at<D: Drawable>(&mut self, drawable: &D, position: Vec2) -> &mut Pencil<'a> {
        let mut new_pencil = self.new_one();
        new_pencil.move_origin(position);
        drawable.draw(new_pencil);
        self
    }

    /// Draws a [Drawable] at (0, 0).
    ///
    /// Returns the receiver for chaining.
    pub fn draw<D: Drawable>(&mut self, drawable: &D) -> &mut Pencil<'a> {
        self.draw_at(drawable, Vec2::zero())
    }
}
