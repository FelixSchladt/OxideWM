.. _about:

=============
About *Oxide*
=============

Introduction
------------
| *Oxide*  windowmanager is a dynamic tiling windowmanager for X11. 
| Windows are automatically arranged in a grid-like fashion. The user can then move and resize windows by using keyboard shortcuts. Custom defining of those shortcuts to launch applications is also possible. 
| *Oxide*  tries to maximize the screensize by removing unnecessary borders and decorations as well as to be as keyboard friendly as possible. Everything can be done via the keyboard.
| *Oxide* is done via a configuration file. The configuration file is written in YAML and can be reloaded at runtime. This makes the user able to change the behavior of *Oxide* without having to restart it.

Terminology
-----------
| **Window**: An X11 application window such as a browser or terminal.
| **Workspace**: A workspace contains multiple windows. The user can switch between several workspaces.
| **Layout**: Layouts are different algorithms placing windows.

Target group
------------
Target group contains power users with advanced Linux knowledge.

Product functions
-----------------
| The *Oxide* window manager gives the user the ability to start and quit applications through its interface. The software itself is supposed to support dynamic tiling, allowing the user to arrange multiple applications in a grid-like arrangement optimizing screen space utilization. Along with this it supports both floating and static applications, giving the user flexibility in his window management.
| Therefore applications are expected to be moved around the screen by the user to different tiled positions or to float as a separate window. 
| Keyboard inputs are handled effectively, allowing the user to control all aspects of the applications by using keyboard shortcuts. The software supports focusing on different windows, making the user able to switch between applications. 
| 
| *Oxide* supports multiple workspaces as well as multiple monitors, allowing the user to create and switch between different virtual desktops and to extend their workspace across multiple screens. It also provides an interface for configuring various settings and options, such as the number of workspaces, monitor arrangement, and more.
| Allowing the user to specify which applications should start automatically when the software is launched is another feature.
| The window manager integrates a taskbar providing an intuitive and streamlined way to switch between open applications and workspaces. For this it is necessary to support popular utilities like Drun or Rofi.
|
| Inter process communication (IPC) is used for interacting between different applications and services, allowing for a seamless integration with the users workflow. The window manager uses a config file in which the user can manage his preferences and settings. Also power management features are included, such as screen locking after a specified timeout to help conserve energy and improve security. For improving the overall user experience the software includes visually appealing animations.

Config file
-----------
*Oxide*  can be configured via its config file. This includes keybindings, appearance and more. Before editing, the global config file located under 

.. code-block:: bash

    /etc/oxide/config.yml

should be copied into the users home directory under 

.. code-block:: bash

    ~/.config/oxide/config.yml

For a more detailed description of the config see :doc:`configuration of Oxide <04_using_oxide/042_config>`.

Logging
-------
*Oxide*  log messages are written to 

.. code-block:: bash
    
        /var/log/syslog

Files
-----
Per-user config file:

.. code-block:: bash

    ~/.config/oxide/config.yml 

Global config file:

.. code-block:: bash

    /etc/oxide/config.yml

*Oxide*  desktop file:

.. code-block:: bash

    /usr/share/xsessions/oxide.desktop

Bugs
----
Please open an issue on https://github.com/DHBW-FN/OxideWM/issues .
