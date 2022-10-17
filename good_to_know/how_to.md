# How to...

## ...install Rust
To install Rust, use one of the following commands depending on your setup:
```bash
sudo pacman -S rustup   # Arch based systems
sudo apt install rustup # Debian based systems
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh  #  Use this if nothing else works
```

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
```
...
[dependencies]
dependency_name: = "dependency version"
somecrate: "0.2.*"
```
