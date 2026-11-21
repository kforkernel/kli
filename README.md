# kli

A simple command-line tool written in Rust for creating lists and saving them to a file.

## Features

* Create lists directly from your terminal.
* Enter as many items as you want.
* Automatically save your list to a specified output file.
* Simple and lightweight, with no external dependencies.

## Requirements

* [Rust & Cargo](https://rust-lang.org)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/kforkernel/kli.git
cd kli
```

2. Build the project:

```bash
cargo build --release
```

## Usage

Run `kli` with an output filename:

```bash
cargo run --release -- shopping.txt
```

Start entering your list items:

```text
Type, press enter on empty line to quit
Milk
Eggs
Bread
Coffee

Saved to shopping.txt
```

Your list will be saved to `shopping.txt`:

```text
Milk
Eggs
Bread
Coffee
```

## License

This project is under the kforkernel Use License (KUL) v1.0.
For more details, read [LICENSE.md](LICENSE.md).
