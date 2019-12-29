use super::terminal::{Canvas, Color, Style};
use super::spatial::{Vec2};

use num::cast::ToPrimitive;

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
    pub fn simple_lines() -> RectCharset {
        RectCharset::from("──││┌┐└┘")
    }

    pub fn simple_round_lines() -> RectCharset {
        RectCharset::from("──││╭╮╰╯")
    }

    pub fn double_lines() -> RectCharset {
        RectCharset::from("══║║╔╗╚╝")
    }
}

impl From<&str> for RectCharset {
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

/*
pub trait Drawable {
    fn draw(&Pencil) {

    }
}
*/

pub struct Pencil<'a> {
    origin: Vec2,
    foreground: Color,
    background: Color,
    style: Style,
    canvas: &'a mut Canvas,
}

impl<'a> Pencil<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Pencil {
        Pencil {
            origin: Vec2::new(),
            foreground: canvas.default_element().foreground,
            background: canvas.default_element().background,
            style: canvas.default_element().style,
            canvas,
        }
    }

    fn draw_element(&mut self, pos: Vec2, value: char) {
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

    pub fn set_origin(&mut self, pos: Vec2) -> &mut Pencil<'a> {
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

    pub fn draw_char(&mut self, value: char, pos: Vec2) -> &mut Pencil<'a> {
        self.draw_element(self.origin + pos, value);
        self
    }

    pub fn draw_text(&mut self, text: &str, pos: Vec2) -> &mut Pencil<'a> {
        let width = self.canvas.dimension().x;
        for (i, value) in text.chars().enumerate() {
            let elem_pos = self.origin + pos + Vec2::x(i);
            let elem_pos = Vec2::xy(elem_pos.x % width, elem_pos.y + elem_pos.x / width);
            self.draw_element(elem_pos, value);
        }
        self
    }

    pub fn draw_vline<T: ToPrimitive>(&mut self, value: char, from: Vec2, size: T) -> &mut Pencil<'a> {
        let elem_pos = self.origin + from;
        for i in 0..size.to_usize().unwrap() {
            self.draw_element(elem_pos + Vec2::y(i), value);
        }
        self
    }

    pub fn draw_hline<T: ToPrimitive>(&mut self, value: char, from: Vec2, size: T) -> &mut Pencil<'a> {
        let elem_pos = self.origin + from;
        for i in 0..size.to_usize().unwrap() {
            self.draw_element(elem_pos + Vec2::x(i), value);
        }
        self
    }

    pub fn draw_rect(&mut self, charset: &RectCharset, origin: Vec2, dimension: Vec2) -> &mut Pencil<'a> {
        let saved_origin = self.origin();
        self.set_origin(saved_origin + origin)
            .draw_hline(charset.top, Vec2::x(1), dimension.x - 2)
            .draw_hline(charset.bottom, Vec2::xy(1, dimension.y - 1), dimension.x - 2)
            .draw_vline(charset.left, Vec2::y(1), dimension.y - 2)
            .draw_vline(charset.right, Vec2::xy(dimension.x - 1, 1), dimension.y - 2)
            .draw_char(charset.top_left, Vec2::xy(0, 0))
            .draw_char(charset.top_right, Vec2::x(dimension.x - 1))
            .draw_char(charset.bottom_left, Vec2::y(dimension.y - 1))
            .draw_char(charset.bottom_right, dimension - Vec2::xy(1, 1))
            .set_origin(saved_origin)
    }
}

impl Clone for Pencil<'_> {
    fn clone(&self) -> Pencil {
        Pencil {
            origin: self.origin,
            foreground: self.foreground,
            background: self.background,
            style: self.style,
            canvas: self.canvas,
        }
    }
}
