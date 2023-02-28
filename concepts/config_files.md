# Config files

At the beginning of the project, it was decided to have an external config file which can be personalised freely.
For this to happen a suitable file format had to be chosen.

## File format

Typical file formats used for config files are `JSON`, `YAML` or `XML`.
`XML` was decided against because of readability.
Compared to `JSON`, comments can be used in `YAML`, so the user can be guided trough the config file and does not need to study all variables and their function
by hard.
Therfore it was decided to use `YAML` as the proper file format for Oxide config files.

## Technical implementation

The following sections describe the technical solutions to enable the use of config files for Oxide.

### Serialization

In order to be able to parse the values of the config file, it has to be serialized and deserialized.
To achive this, `serde_yaml` can be used. The `serde_yaml` crate is a rust library and therfore an optimal solution for the parser.

### Default values

If the user does not create his own config file, it was necessary to have some default values in order for Oxide to work. `serde_yaml` implements the following
function.

```rust
#[serde(default='default_value')]
```

Using this it is possible to use default values for not present or wrongly assigned variables.
