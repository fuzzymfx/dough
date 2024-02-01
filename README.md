# Dough

Dough is a command-line tool written in Rust that allows you to create and present presentations using Markdown. It provides a simple and customizable way to create presentation decks and present them in both terminal and HTML modes.

<img width="1840" alt="Screenshot 2024-01-30 at 11 47 43 AM" src="https://github.com/fuzzymfx/dough/assets/69160388/8351ee95-0589-46ff-ae72-5ab68bab6c03">


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

Dough provides two main subcommands: new and present.

#### Creating a New Project

```bash
dough new <project-name> [--template <template-name>]
```

`<project-name>`: The name of the new project (required).
--template `<template-name>`: Choose a template for the project (default is "default").
Example:

```bash
dough new my_presentation --template fancy_template
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

#### Navigating through the Presentation

You can use arrow keys to navigate: right and left arrows to move between slides, and up and down arrows to move between lines.
Vim keybindings are also supported: `h` and `l` to move between slides, and `j` and `k` to move between lines.
The letters are case-insensitive.
`q` to quit the presentation.

## Contributing

If you're familiar with Rust and are looking for a project to contribute to, Dough would be a decent starting point. Feel free to open an issue or submit a pull request.

TODO:

- Bugs:
- [ ] Improving the rendering engine:
  - [x] Add support for rendering **nested syntax**
  - [ ] **Fix the color storage** of multiline elements while parsing a line and stripping the ANSI escape sequences.
  - [ ] Improve multi markdown element rendering (e.g. link inside a heading or a list)
    - [x] Improve the rendering of lists, especially nested lists
    - [ ] Improve the rendering of headings inside blockquotes
  - [ ] Improve color correction after alignment
  - [ ] Add proper support for common Markdown syntax
    - [x] Improve the rendering of thematic breaks

- Features:
  - [x] Improve the design language
  - [ ] Add support for images for terminals with image support
- [ ] Syntax Highlighting in code blocks
- [x] Custom text alignment
  - [x] A regex match for individual text alignment - `[\c] - for center, [\l]\ (blank) -default - for left, [\r] - for right`  
- [ ] Running code blocks on separate threads and outputting the results in the current console

## Acknowledgements

Authors: [fuzzymfx](https://github.com/fuzzymfx), [injuly](https://github.com/injuly)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
