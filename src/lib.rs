//! # `ruscii`
//!
//! `ruscii` is a library that aims to provide a fast, straightforward, and easy-to-use interface
//! with the terminal to create text-based games and applications. It can be used on multiple
//! platforms and terminals. For more information, see
//! [Crossterm's terminal support](https://github.com/crossterm-rs/crossterm#tested-terminals).
//!
//! `ruscii`'s simple API for key press/release events and drawing on the terminal screen enables
//! effortless and quick game development in relatively few lines. For examples, see the
//! [examples](https://github.com/lemunozm/ruscii/tree/master/examples) folder of the
//! [ruscii](https://github.com/lemunozm/ruscii) repository.

pub mod app;
pub mod gui;
pub mod terminal;
pub mod keyboard;
pub mod spatial;
pub mod drawing;
