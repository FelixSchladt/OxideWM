.. _intro_installation:

============
Installation
============

Prerequisits
------------
Rust needs to be installed. After it has been installed, restart the terminal session, so that any new environment variables are loaded.

.. code-block:: bash

        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


Build tools need to be install:

.. code-block:: bash

        sudo apt install git make build-essential libglib2.0-dev libcairo2-dev libpango1.0-dev kitty xterm


Installation
------------

1. Clone the *Oxide* git repository:

.. code-block:: bash

        git clone https://github.com/DHBW-FN/OxideWM.git

2. Install *Oxide* via make:

.. code-block:: bash
        
        cd OxideWM
        make install

| Sudo privileges are required to install *Oxide*. 
| After installation you can quit your current X session and log out. Subsequently *Oxide* should be selectable as window manager in your login screen.


