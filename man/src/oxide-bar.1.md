% OXIDE-BAR(1) oxide-bar 0.1.0
% Philipp Kalinowski
% February 2023

# NAME

oxide-bar - statusbar for Oxide

# DESCRIPTION

Define the behavior of the statusbar for Oxide. The config file provides the possibility to customize the text and background color of the oxide statusbar.
The config file is written in YAML.

# FILES

During launch, Oxide bar searches for a config file in the following locations:

**~/.config/oxide/bar_config.yml**
: Home config file

**/etc/oxide/bar_config.yml**
: System config file

# COLORS

In order to configure the colors, they have to be entered in hexadecimal. If the colors are not defined, default values will be used.

# EXAMPLES

```yaml
color_bg: "0x008000" # green
color_txt: "0xFFFF00" # black
```

# Bugs

Please open an issue <https://github.com/DHBW-FN/OxideWM/issues> .

# COPYRIGHT

Copyright © 2023 Philipp Kalinowski GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: You are free to change and redistribute it. There is NO WARRANTY to the extent permitted by law.

# FURTHER DOCUMENTATION

Access the full Oxide documentation under **https://oxide.readthedocs.io/**.

# SEE ALSO

**oxide(1)**, **oxide-msg(1)**, **oxide-bar(1)**, **oxide-config(1)**
