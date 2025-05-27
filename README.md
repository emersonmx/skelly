# Skelly

A simple scaffold tool written in Rust. Skelly helps you quickly set up new
project structures or generate files from templates.

## Features

*   **Project Scaffolding**: Generate entire project structures from a skeleton
    directory.
*   **Single File Rendering**: Render individual template files.
*   **Standard Input/Output**: Process templates from standard input and write
    to standard output.
*   **Flexible Inputs**: Pass custom key-value inputs to templates.
*   **Input Validation**: Define required inputs and options in `skelly.toml`.
*   **Tera Templating**: Utilizes the Tera templating engine for powerful
    template syntax.

## Installation

You can install `skelly` using `cargo`:

```bash
cargo install --git https://github.com/emersonmx/skelly
```

## Usage

### Scaffolding a Project

To scaffold a new project from a skeleton directory (e.g., `my-skeleton/`):

```bash
skelly \
    --skeleton-path my-skeleton \
    --output-path my-new-project \
    build_tool=cargo target_path=dist
```

This command will:
1.  Look for `skelly.toml` inside `my-skeleton/`.
2.  Read the templates from `my-skeleton/skeleton/`.
3.  Process all files and directories within `my-skeleton/skeleton/`, rendering
    their content and filenames using the provided inputs.
4.  Write the rendered output to `my-new-project/`.

If `output-path` is omitted, the current directory (`.`) is used.

To print the scaffolded project to stdout:

```bash
skelly \
    --skeleton-path my-skeleton \
    build_tool=cargo target_path=dist > project.txt
```

### Rendering a Single File

To render a single file using `skelly`:

```bash
skelly --file-path template.txt name=World
```

This will render `template.txt` using the `name=World` input and print the
result to standard output.

### Rendering from Standard Input

You can pipe content to `skelly` to render it:

```bash
echo "Hello {{ name }}!" | skelly name=Alice
```

This will output `Hello Alice!` to standard output.

### Inputs

Inputs are passed as `KEY=value` pairs after other arguments. These inputs are
used to fill in placeholders in your templates.

```bash
skelly ... my_key="my value" another_key=123
```

### Verbose Output

Use the `-v` or `--verbose` flag for more detailed error messages.

```bash
skelly -v --file-path template.txt
```

## Skeleton Configuration (`skelly.toml`)

When scaffolding a project using `--skeleton-path`, `skelly` expects a
`skelly.toml` file inside the specified directory. This file defines the inputs
your skeleton expects and configures the template directory.

Example `skelly.toml`:

```toml
[[inputs]]
name = "build_tool"
options = ["rustc", "cargo"]
default = "cargo"

[[inputs]]
name = "target_path"
default = "target"
```

*   `template_directory`: (Implicitly handled, defaults to a `skeleton`
    subdirectory inside the skeleton path).
*   `inputs`: An array of input definitions.
    *   `name`: The name of the input (e.g., `build_tool`).
    *   `options`: An optional list of allowed values for the input. If
        provided, `skelly` will validate the user-provided input against these
        options. If `options` is an empty list, any value is accepted.
    *   `default`: An optional default value for the input. If the user doesn't
        provide this input, the default value will be used. Integers, booleans,
        and other types provided as `default` will be converted to strings.

The actual template files and directories should be placed in a subdirectory
named `skeleton` (by default) within your skeleton path. For example, if your
skeleton path is `my-project-template/`, your templates would reside in
`my-project-template/skeleton/`.

## Templating

Skelly uses the [Tera](https://keats.github.io/tera/) for rendering. You can use
standard Tera syntax for variables, loops, conditionals, etc., within your
template files and even in the names of your files and directories.

**Example `src/main.rs` template:**

```rust
fn main() {
    println!("Hello, {{ name }}!");
}
```

When rendering with `name=World`, the output will be `Hello, World!`.

## License

`Skelly` is licensed under the MIT License. See the `LICENSE` file for more
details.
