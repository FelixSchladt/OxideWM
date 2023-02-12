% OXIDE-BAR(1) oxide-bar 0.1.0
% Philipp Kalinowski
% February 2023

# NAME

oxide-bar - statusbar for Oxide

# DESCRIPTION

Define the behavior of the statusbar for OxideWm. You can change the colors, heigth and the modules wich are displayed left and right on the statusbar.
As the config file for oxide, the config for the statusbar is written in YAML.

# FILES

When starting, Oxide has to paths it searches for the statusbar config file.

**~/.config/oxide/bar_config.yml**
: Home config file

**/etc/oxide/bar_config.yml**
: System config file

# LAYOUT

TODO: Is this even necesssary?

You can only customise the height of the statusbar, since the weight is depending on the windows an will be defined trougth Oxide.

# COLORS

You can freely customise the colors of the satus bar. Inside the configuration file, colors have to be entered in hexadecimal.
If there is no configuration for the colors, the default colors will be used, whereas black is used for the background and white for the text.

# MODULES

TODO: List alls modules available for oxide-bar.

# EXAMPLES

TODO: Add examples

# Bugs

Please open an issue <https://github.com/DHBW-FN/OxideWM/issues> .

# COPYRIGHT

Copyright © 2023 Philipp Kalinowski GPLv3+\: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: You are free to change and redistribute it. There is NO WARRANTY to the extent permitted by law.

# FURTHER DOCUMENTATION

Access the full Oxide documentation under **https://oxide.readthedocs.io/**.

# SEE ALSO

**oxide(1)**, **oxide-msg(1)**, **oxide-bar(1)**, **oxide-config(1)**
