# oxide-msg

The `oxide-msg` is an ipc command line and library tool. It aims to provide an easy to use tool to control the window manager via scripts or code.


## Usage

In debug mode directly from source:
```sh
cargo run -p oxide-msg -- exec kitty
```


After install:
```
oxide-msg kill
```

for more information run:

```sh
cargo run -p oxide-msg -- --help
```

## Module

The `oxide-msg` tool is part of this repository as its own workspace and can be found under `extensions/oxide-msg`.
All the typical cargo functionality is available via the `cargo <cmd> -p oxide-msg`.
