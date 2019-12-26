[![lancat](https://img.shields.io/crates/v/ruscii)](https://crates.io/crates/ruscii)
[![license](https://img.shields.io/crates/l/ruscii)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![downloads](https://img.shields.io/crates/d/ruscii)](https://crates.io/crates/ruscii)

# `ruscii`
An easy library to make terminal applications/games in *rust*.

The intended of this project is to ease the game development in terminals.
Any contribution, issue, or pull request is welcome!

### Features
- Optimized to render fast in terminals.
- Multi-platform (See [crossterm terminal support](https://github.com/crossterm-rs/crossterm#tested-terminals))
- Enable **key press** and **release** events in terminal (essential for games!)
- Easy to use. Make your terminal game in a few lines!
- Easy way to recover the terminal state at error.

## Examples
You can found several examples into the [example folder](examples).

To test the example, clone the repo and write:
```
cargo run --example <example_file_name_without_extension> --release
```

### Some of these examples:
#### Space invaders ([200 lines](examples/space_invaders.rs)):
  ![](images/space_invaders.png)

#### Pong ([150 lines](examples/pong.rs)):
  ![](images/pong.png)

