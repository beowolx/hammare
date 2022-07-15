# Hammare 🔨⚡️ - a blazing fast text editor written in Rust 

Hammare (hammer in swedish) is a simple text editor inspired by `vim`. It lets you
perform simple text edition operations directly in the terminal.

![screen-gif](./hammareDemo.gif)

## How to install it

You are going to need to install Rust and Cargo in your computer.

You can use [rustup](https://rustup.rs/) to install the toolchain.

## How to use it

Clone this project and go to the directory:

```bash
git clone https://github.com/LuisCardosoOliveira/hammare.git
cd hammare
```

Then, you can build it and run it using `cargo`:

```bash
cargo build --release
cargo run
```

It should prompt you with an empty file that you can edit and save.

To edit an existing file, you need to pass the file name as an argument, for example:

```bash
cargo run file.rs
```

## Supported commands

For the moment, `hammare` only supports a few commands, but I'm working everyday
to improve it:

- `Ctrl + S` -> Save your changes/file
- `Ctrl + T` -> Exit the editor





