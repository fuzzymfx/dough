# Dough

Dough is a command-line tool written in Rust that allows you to create and present presentations using Markdown. It provides a simple and customizable way to create presentation decks and present them in both terminal and HTML modes.

<img width="651" alt="introduction" src="https://github.com/fuzzymfx/dough/assets/69160388/3fdb4113-6fe8-4b49-816b-56097c9a1fdd">

## Getting Started

### Installation

### Binary

Get the latest release from the [releases page](https://github.com/fuzzymfx/dough/releases).

create a project directory and write your presentation in markdown.

```bash
mkdir my_folder && touch my_folder/1.md
```

```bash
./dough present <project-name> 
```

### Working with Source

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

#### Without templates

Create a new dir and create your presentations using markdown:

```bash
dough present <project-name> [directory name]
```

Dough provides two main subcommands: new and present. 

#### Creating a New Project

#### With templates

```bash
dough new <project-name> [--template <template-name>]
```

<img width="422" alt="example" src="https://github.com/fuzzymfx/dough/assets/69160388/ecfb47c2-8fa9-4cb4-9425-0283e0675a6e">

`<project-name>`: The name of the new project (required).
--template `<template-name>`: Choose a template for the project (default is "default").
Example:

```bash
dough new my_presentation --template fancy_template
```

Currently, there are three templates:

- `default`
- `code`
- `light`

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

- `t` :
  - `highlighting` mode
  - `scrolling` mode
- `q`, `Esc`, or `ctrl + c` to quit the presentation.
- `ctrl + r` to refresh the presentation.
- `l` or `right arrow` to move to the next slide.
- `h` or `left arrow` to move to the previous slide.
- `j` or `down arrow` to move to the next line.
- `k` or `up arrow` to move to the previous line.

The `t` key is used to toggle between **highlighting** and **scrolling** modes. In highlighting mode, you can use the arrow keys to navigate between slides. In scrolling mode, you can use the arrow keys to scroll through the content of the current slide.

You can use arrow keys to navigate: right and left arrows to move between slides, and up and down arrows to move between lines.
Vim keybindings are also supported: `h` and `l` to move between slides, and `j` and `k` to move between lines.
The letters are case-insensitive.

#### Customizing the Presentation

You can customize the presentation by modifying the `style.yml` file in the project directory. The `config.yaml` file contains the default style settings for the terminal markdown renderer.

```yaml
# This file contains the default style settings for the terminal markdown renderer.

# The highlighter
highlighter: black on white

# Markdown styles
h1: red
h2: yellow
h3: green
h4: cyan
h5: blue
h6: purple
blockquote: white on black
ordered_list_bullet: yellow
unordered_list_bullet: yellow
ordered_list: white
unordered_list: white
link_text: black
link_url: blue
definition: yellow

# Terminal styles

# clear will clear the terminal before rendering, you would need to scroll down to render each line
clear: false

box: true
box_color: black on white

# vertical_alignment will vertically align the text to the middle of the terminal
vertical_alignment: true

# horizontal_alignment will horizontally align the text to the middle of the terminal
horizontal_alignment: true

# syntax_highlighting will highlight the code syntax
# this works well with the warp terminal, but not with the default Mac OS terminal

syntax_highlighting: true
syntax_theme: base16-ocean.dark
#themes:[base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light, Solarized (dark) and Solarized (light)]
syntax_bg: false

# shows the progress of the presentation: " i [1/10] " : current slide/total slides in the bottom left corner
progress: false

# runtime map is used to store the runtimes for different languages
# you can add your own runtimes for different languages. Currently, the following runtimes are supported:

- runtime_map:
    python: python3
    javascript: node
    ruby: ruby
    c: gcc
    cpp: g++
    java: java
    go: go run
    rust: cargo run
    swift: swift
    kotlin: kotlinc
    typescript: tsc
```


#### Running Code Blocks

Dough supports running code blocks in the terminal. The code blocks are internally ordered in the order they appear in the markdown file. The code blocks are run in a separate thread, and the results are displayed in the terminal.

- `n`: runs the `n`th code block. and outputs the result in the terminal.

## Contributing

If you're familiar with Rust and are looking for a project to contribute to, Dough would be a decent starting point. Feel free to open an issue or submit a pull request.

TODO:

<img width="560" alt="syntax-highlighting" src="https://github.com/fuzzymfx/dough/assets/69160388/8b071096-adce-434c-8ff5-5204e6ca19b1">

- [ ]  Scrolling issues in highlight mode:
  - [ ] *bug* Over Scroll in infinite scrolling 
  - [ ] *feat.* Skip empty lines in both highlight and scroll modes
- [ ] *feat.* Hot Module Reload
- [ ] *feat.* Add a support for maximum width and height of the terminal. Write a word wrapper.
- [ ] *feat.* Add comprehensive support for common Markdown elements.
- [ ] *feat.* Enhance rendering for complex markdown elements
  - headings withing blockquotes
  - links within headings or lists 
- [ ] *feat.* Image support for terminals with image capabilities *(Kitty, iTerm2, etc.)*


## Acknowledgements

Authors: [fuzzymfx](https://github.com/fuzzymfx), [injuly](https://github.com/injuly)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
