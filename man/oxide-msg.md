% OXIDE-MSG(1) oxide-msg 0.1.0
% Felix Schladt
% February 2023

# NAME
oxide-msg - ipc



# oxide-msg

The `oxide-msg` is an ipc command line and library tool. It aims to provide an easy to use tool to control the window manager via scripts or code.


## Usage

```sh
cargo run -p oxide-msg -- -c "exec" -a "kitty"
cargo run -p oxide-msg -- --command "kill"
```

for more information run:

```sh
cargo run -p oxide-msg -- --help
```

## Module

The `oxide-msg` tool is part of this repository as its own workspace and can be found under `tools/oxide-msg`.
All the typical cargo functionality is available via the `cargo <cmd> -p oxide-msg`.
