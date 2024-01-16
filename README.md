# Dough

Dough is a command-line tool written in Rust that allows you to create and present presentations using Markdown. It provides a simple and customizable way to create presentation decks and present them in both terminal and HTML modes.

## Getting Started

### Prerequisites

- [Rust programming language](https://www.rust-lang.org/tools/install)
- Clap library: clap = "2"
- Paris library: paris = "0.5"

### Installation

Clone the Dough repository:

```bash
git clone https://github.com/fuzzymfx/dough.git

```

Navigate to the Dough project directory:

```bash
cd dough
```

Build the project using Cargo:

```bash
cargo build --release
```

Install the binary:

```bash
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"

```

### Usage

Dough provides two main subcommands: create and present.

#### Creating a New Project

```bash
dough create <project-name> [--template <template-name>]
```

`<project-name>`: The name of the new project (required).
--template `<template-name>`: Choose a template for the project (default is "default").
Example:

```bash
dough create my_presentation --template fancy_template
```

You can add new `templates` under the templates folder. Each template is a folder containing a `template.md` file that contains the template's Markdown code.
<!-- You can also add a `template.css` file to add custom CSS styling to the template. -->

#### Presenting a Deck

```bash
dough present <project-name> [--mode <presentation-mode>]
```

`<project-name>`: The name of the project to present (required).
--mode `<presentation-mode>`: Choose the mode of presentation: "html" or "term" (default is "term").
Example:

```bash
dough present my_presentation --mode html
```

## Contributing

TODO:

- [x] Proper support for line breaks and spaces.
- [ ] Improve the rendering engine:
  - [ ] handle all possible Markdown elements
- [ ] Rendering style adjustment: differntiating different types of headings using color etc. and alignment
- [ ] Full arrow key support + more hotkeys
- [ ] Syntax Highlighting

## Acknowledgements

Authors: [fuzzymfx](https://github.com/fuzzymfx), [injuly](https://github.com/injuly)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details