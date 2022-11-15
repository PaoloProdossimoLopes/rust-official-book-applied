# Add and Update Depdendencie on Cargo
To add new pendencie your need to open file named `Cargo.toml`, are similar that example:
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

for to add new pendencie add bellow `[dependencies]` with name and version your file will became similar that
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.5.5"
```

## How to update:
1. Add new pendencie in `Cargo.toml`.
2. Run `cargo update`, this command will download somethig is nescessary to compile.
3. [OPTIONAL] Run `cargo run`, this command ifs nescessary because at some moment, in this step is possible the cargo install mroe things and prepare to run existing project. 