# pGLOWrpg - Procedurally generated living open world RPG
![Banner][splash]

## Features
![Some progress][animation]

* World generation with optional image maps output (colorized and raw).
* World navigation (with redundant CLI rendering for debugging).

Feel free to play with world presets to find out interesting options combinations.
The ***presets_user*** folder is ignored by git, so store your custom presets there.

If the **worldgen** module is updated, the compatibility with saves might be broken
until save data file version check is implemented.

## Requirements
- rust (https://www.rust-lang.org/)

## Getting started
It is quite straightforward when working on a computer (rust must be installed), just do this:
- In the root directory execute `cargo run --release`
- Or use the "pglowrpg.sh" shell script which does the same

You must run pGLOWrpg from the folder that contains "options", "presets", "locale" and "save"
folders, so there is no need to do "cargo install".

[splash]: doc/images/pglowrpg_banner.png "Banner"
[animation]: doc/images/pglowrpg_progress.gif "Animation"