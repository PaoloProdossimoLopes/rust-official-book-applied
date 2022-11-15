# Runing your file (or project)
To runs Rust's application file should be follow some process

## Overview:
Imagine we want to create a project to print `hello word`, therefore we will creta a folder
named 'hello_word_project' and inside of this folder should be nescessary create the `main.rs` file
with code bellow:

```rust
fn main() {
  println!("hello world")
}
```

## Steps to run
1. Nagivate to file in the terminal.
2. run `rustc main.rs` to compile 'main.rs' file.
3. run `main` to run script generated in the last step.