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
*   **Input Validation**: Define required inputs, defaults and allowed options
    in `skelly.toml`.
*   **Template Libraries**: Make extra templates available to `include`,
    `extends` and `import` when rendering a file or standard input.
*   **Tera Templating**: Uses the Tera templating engine, extended with the
    `tera-contrib` filters, tests and functions.

## Installation

You can install `skelly` using `cargo`:

```bash
cargo install --git https://github.com/emersonmx/skelly
```

## Usage

```
Usage: skelly [OPTIONS] [INPUTS]...

Arguments:
  [INPUTS]...  Inputs passed to the skeleton

Options:
  -s, --skeleton-path <DIRECTORY>  Which skeleton to use
  -f, --file-path <FILE>           Which file to use
  -o, --output-path <DIRECTORY>    Where to output the generated skeleton into [default: .]
  -l, --library <PATH|NAME=PATH>   Templates available to the main template
  -h, --help                       Print help
  -V, --version                    Print version
```

Skelly picks a mode from the arguments:

| Arguments          | Mode                                                        |
| ------------------ | ----------------------------------------------------------- |
| `--skeleton-path`  | Scaffold a skeleton into `--output-path` (or stdout if piped) |
| `--file-path`      | Render a single file to stdout                              |
| neither            | Render standard input to stdout                             |

`--skeleton-path` and `--file-path` can't be combined, and neither can be
combined with piped standard input.

### Scaffolding a Project

To scaffold a new project from a skeleton directory (e.g., `my-skeleton/`):

```bash
skelly \
    --skeleton-path my-skeleton \
    --output-path my-new-project \
    build_tool=cargo target_path=dist
```

This command will:
1.  Read and validate the inputs against `my-skeleton/skelly.toml`.
2.  Walk every file inside `my-skeleton/skeleton/`.
3.  Render each file's content and its relative path using the inputs.
4.  Write the rendered files under `my-new-project/`, creating directories as
    needed.

If `--output-path` is omitted, the current directory (`.`) is used. The output
directory is created if it doesn't exist.

When standard output is not a terminal, the rendered contents of all files are
printed to standard output instead of being written to disk:

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

Inputs are passed as `KEY=value` pairs after the other arguments. They fill in
the placeholders in your templates.

```bash
skelly ... my_key="my value" another_key=123
```

All values are passed to templates as strings. When scaffolding a skeleton,
only inputs declared in `skelly.toml` are used; unknown inputs are ignored.
Rendering a file or standard input uses every input given.

Referencing a variable that wasn't provided is an error, and Skelly reports
where it happened in the template.

### Libraries

Use `--library` (`-l`) to make extra templates available to the main template,
so it can `include`, `extends` or `import` them. Libraries work with
`--file-path` and standard input; they can't be combined with
`--skeleton-path`. The option may be repeated.

The template name depends on the form used:

| Argument                 | Template name(s)                                 |
| ------------------------ | ------------------------------------------------ |
| `-l path/greet.txt`      | `greet.txt` (the file name)                      |
| `-l path/partials`       | `partials/<relative path>` for every file inside |
| `-l greet=path/file.txt` | `greet`                                          |
| `-l ui=path/partials`    | `ui/<relative path>` for every file inside       |

```bash
echo 'Hi {{ name }}' > partials/greet.txt
echo '{% include "partials/greet.txt" %}!' | skelly -l partials name=Ana
```

## Skeleton Configuration (`skelly.toml`)

When scaffolding a project using `--skeleton-path`, `skelly` expects a
`skelly.toml` file inside the specified directory. This file defines the inputs
your skeleton expects.

Example `skelly.toml` (see [`examples/skelly.toml`](examples/skelly.toml)):

```toml
[[inputs]]
name = "build_tool"
options = ["rustc", "cargo"]
default = "cargo"

[[inputs]]
name = "target_path"
default = "target"
```

*   `inputs`: An array of input definitions.
    *   `name`: The name of the input (e.g., `build_tool`).
    *   `options`: An optional list of allowed values for the input. If
        provided, `skelly` validates the user-provided input against these
        options. If `options` is an empty list, any value is accepted.
    *   `default`: An optional default value, used when the user doesn't
        provide the input. Integers, booleans and other types are converted
        to strings.

An input without a `default` is required. All validation errors are reported
at once, e.g.:

```
Invalid option 'make' to input 'build_tool'. Available options: rustc, cargo.
Missing input 'project_name'.
```

The template files and directories must be placed in a subdirectory named
`skeleton` within your skeleton path. For example, if your skeleton path is
`my-project-template/`, your templates reside in
`my-project-template/skeleton/`:

```
my-project-template/
├── skelly.toml
└── skeleton/
    ├── Cargo.toml
    └── src/
        └── {{ project_name }}.rs
```

## Templating

Skelly uses [Tera](https://keats.github.io/tera/) for rendering, with
autoescaping disabled. You can use standard Tera syntax for variables, loops,
conditionals, etc., within your template files and even in the names of your
files and directories.

**Example `src/main.rs` template:**

```rust
fn main() {
    println!("Hello, {{ name }}!");
}
```

When rendering with `name=World`, the output will be `Hello, World!`.

On top of Tera's built-ins, the following
[`tera-contrib`](https://crates.io/crates/tera-contrib) extras are available:

*   **Filters**: `b64_encode`, `b64_decode`, `date`, `filesize_format`,
    `format`, `json_encode`, `shuffle`, `striptags`, `spaceless`,
    `regex_replace`, `slug`, `urlencode`, `urlencode_strict`.
*   **Tests**: `before`, `after`, `matching`.
*   **Functions**: `now`, `get_random`.

```bash
echo '{{ name | slug }}' | skelly "name=Hello World"   # hello-world
```

## Development

The project uses [`just`](https://github.com/casey/just) as a task runner:

```bash
just setup        # install lefthook, bacon and cargo-insta
just build        # cargo build
just test         # cargo test
just lint         # cargo clippy
just format       # cargo fmt
just watch        # bacon
just review-snap  # review insta snapshots
```

## License

`Skelly` is licensed under the MIT License. See the `LICENSE` file for more
details.
