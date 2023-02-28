# Window Manager Configuration

At the beginning of the project, it was decided to have an external config file which can be personalised freely.
For this to happen a suitable file format had to be chosen.

## File format

Typical file formats used for config files are `JSON`, `YAML` or `XML`.
Due to poor readability, `XML` has been ruled out from the start.
Unfortunately does `JSON` not support any comments inside the file, which was decided to be an important feature.
Therefore it was decided to use `YAML` as the proper file format for **Oxide** config files.

## Technical implementation

The following sections describe the argument for the chosen parsing library.

### Parsing the config file

The config file needs to be parsed before we can accesses the stored data.
This should be as easy and effortless as possible. The preferred solution for this is to have a parser that outputs a single struct which contains all config
values.

### Library

The serde crate is the obvious choice for serialization and deserialization in the rust eco-system. It is widely supported and has subcrates such as serde_yaml
for specific file formats.

Additionally features such as giving fiels default values when not part of the config are possible.

```rust
#[serde(default='default_value')]
```

Using this it is possible to use default values for not present or wrongly assigned variables.

## Conclusion

After evaluating all aspects the team came to the conclusion to use YAML as file format and the serde_yaml crate as parser.
