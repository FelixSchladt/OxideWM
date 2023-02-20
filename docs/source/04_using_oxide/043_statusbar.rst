.. _statusbar:

==========================
Configuration of statusbar
==========================

Description
-----------

Define the behavior of the statusbar for *Oxide*. The config file provides the possibility to customize the text and background color of the *Oxide* statusbar.
The config file is written in YAML.

Files
-----

During launch, *Oxide* bar searches for a statusbar config file in the following two locations.

**Home config file:**

.. code-block:: bash
    
    ~/.config/Oxide/bar_config.yml

**System config file:**

.. code-block:: bash

    /etc/Oxide/bar_config.yml

Color
-----

In order to configure the colors, they have to be entered in hexadecimal. If the colors are not defined, default values will be used.

Examples
--------

.. code-block:: bash

    color_bg: "0x008000" # green
    color_txt: "0xFFFF00" # black

Bugs
----

Please open an issue https://github.com/DHBW-FN/OxideWM/issues .



