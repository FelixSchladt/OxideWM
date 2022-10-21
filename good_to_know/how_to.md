# How to...

## ...install Rust
To install Rust, use one of the following commands depending on your setup:

### Arch
```bash
sudo pacman -S rustup   # Arch based systems
```

### Debian
```bash
sudo apt install rustup # Debian based systems
```

### POSIX
Should work on all posix compliant system. 
The disadvantage is that the packet is not maintained and updated by your system package manager.
```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh  #  Use this if nothing else works
```

## ..learn rust
The rust project provides some great free documentation and trainings to get started with rust
[Official: Learn Rust](https://www.rust-lang.org/learn)
[Rust Book](https://doc.rust-lang.org/book/)
[Rust training course](https://github.com/rust-lang/rustlings/)


## ...start a project
To start a rust project, run the command
```bash
cargo init # Create a rust project
```

## ...build your project
Build a project with
```bash
cargo build # Build for debugging
cargo build --release # Build for release, this performs some additional optimizations
cargo check # Check for errors without building
```

## ...run a rust project
```bash
cargo run
```

## ...add dependencies to a rust project
Add dependencies by editing `Cargo.toml`.
Add the following lines:
```toml
[dependencies]
somecrate = "0.2.*"
```
