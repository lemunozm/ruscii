[![lancat](https://img.shields.io/crates/v/ruscii)](https://crates.io/crates/ruscii)
[![license](https://img.shields.io/crates/l/ruscii)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![downloads](https://img.shields.io/crates/d/ruscii)](https://crates.io/crates/ruscii)

# `ruscii`

An easy-to-use library for writing terminal applications and games in Rust.

The aim of this project is to make text-based game development straightforward
and effortless.
Any contribution, issue, or pull request would be greatly appreciated!

## Features

- Optimized to render fast in terminals
- Multiplatform (Linux, Windows and macOS)
    - For Linux, it is required to have a x11 server (most distributions come with one included).
      Internally, `ruscii` uses it to create transparent key-pressed and key-released events.
- Support for multiple terminals (
  See [Crossterm's terminal support](https://github.com/crossterm-rs/crossterm#tested-terminals))
- Provides key press and release events in terminal (essential for games!)
- Simplistic API - make your terminal-based game in relatively few lines!
- Provides an easy way to recover the terminal in an error state

## Dependencies

### Linux

To compile applications written using `ruscii` in Linux, you must have the X11 development libraries installed.

If your system does not have them, you can use the following commands in the terminal to install them according to your
specific Linux distribution.

- In Ubuntu or Debian:
  ```sh
  sudo apt install libx11-dev
  ```
- In Fedora, RHEL, or CentOS:
  ```sh
  sudo dnf install xorg-x11-server-devel
  ```

### Windows and macOS

Windows and macOS have no special dependencies.

## Examples

You can find several examples of applications and games written using `ruscii` in the [examples folder](examples).

To test one out, install `ruscii` with the examples flag using the following command.

```
cargo install ruscii --examples
~/.cargo/bin/<example_name>
```

After it's installed, run it with:

```sh
cargo run --example <example_name> --release
```

### Example Games

#### Space Invaders ([200 lines](examples/space_invaders.rs))

[![asciicast](https://asciinema.org/a/291004.svg)](https://asciinema.org/a/291004)

#### Pong ([150 lines](examples/pong.rs))

[![asciicast](https://asciinema.org/a/291007.svg)](https://asciinema.org/a/291007)

> Note: the first `asciimedia` playback might be laggy; playing it a second time fixes this issue.

### Projects Using `ruscii`

- [thrust](https://github.com/matwoess/thrust) - A simple space shooter game. Runs in the terminal using character-based
  UI.
- [terminal-tetris](https://github.com/joinemm/terminal-tetris) - 🕹️ Tetris in the terminal written in Rust.

*If you have a project using `ruscii` and would like it to appear here, open an issue!*

## Getting Started

### Installation

Add the following line to the `[dependencies]` section in your `Cargo.toml` file:

```toml
ruscii = "0.3.2"
```

### Test it in your own terminal!

Copy the following code to your `main.rs` file to run the base `ruscii` application:

```rust
use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
    });
}
```

### Debugging

Debugging a terminal app is usually difficult because the app output and the backtrace goes to the same terminal view.
`ruscii` uses the _standard output_ to render your app and the _standard error_ to log error information.
We recommend that you redirect the _standard error_ to a file to be inspected later.

To run your project with error logs saved to a file, run the following commands:

```sh
export RUST_BACKTRACE=1
cargo run 2> my_stderr
```

All error output will be saved to `my_stderr` in the project directory.