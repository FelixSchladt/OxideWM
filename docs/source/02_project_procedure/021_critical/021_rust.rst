.. _rust:

===============================
Rust as implementation language
===============================

This document outlines why rust has been chosen as the language of
implementation for this project. But it needs to be mentioned that this
choice was not purely made out of technical reasoning, because we think
rust is fun and we enjoy to learn this language.

Technical arguments for Rust
----------------------------

Rust is a good choice for a system level language as it allows direct
memory acces but also object orientation and offers a memory safe
environment. Further this is achieved without a resource intensive
garbage collection and high speeds in the realm of C are possible. Even
though rust is still “relatively” new, it already has a stable ecosystem
and a lot of libraries supporting it.

A first search for rust libraries allowing connections to the X-Server
shows multiple results. The same is true for dbus- and IPC-libraries.

Therefore Rust seems to be a good choice for our project.
