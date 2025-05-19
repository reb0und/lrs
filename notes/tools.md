# Tools

### Entrypoint
The entrypoint to a Rust program is a function ```fn main()```

### Tools
#### `rustup`
- Manages Rust versions
#### `rustc`
- Rust compiler
- Usage:
    - `rustc <filename.rs>`
- Similar to gcc or clang

### Hello World
- `println!()` is a macro
    - `!` indicates macro
    - Macros are way to write code that generates code to extend to Rust syntax, covered in Chapter 20
- `;` indicates expression is over and next one begins

### Cargo
- Rust build system and package manager
#### Usage
- Create new project with `cargo new <project_name>`
    - Creates new directory with project with VCS unless already within project (can be overriden with `cargo new --vcs=git`)
- Creates a `Cargo.toml` file specifying project info and dependencies (crates)
- Can build and run code via `cargo run`

##### Building
- `cargo build` creates and outputs binary at `target/debug/{binary}`
- `cargo check` checks if code will compile, much faster than building
- `cargo build --release` builds for release with optimizations


