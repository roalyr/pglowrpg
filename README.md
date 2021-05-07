# pGLOWrpg - Procedurally generated living open world RPG
![Banner][splash]

## Features
![Biomes map of some medium continent][biomes]

![Worldgen UI sample][ui_sample]

![Some redundant rendering][animation]

* World generation with optional image maps output (colorized and raw).
* World navigation (with redundant CLI rendering for debugging).

Feel free to play with world presets to find out interesting options combinations.
The ***presets_user*** folder is ignored by git, so store your custom presets there.

If the **worldgen** module is updated, the compatibility with saves might be broken
until save data file version check is implemented.

## Requirements
Main requirements:
- rust (https://www.rust-lang.org/)

Optional requirements (for the "common actions" wrapper):
- python3
- graphviz (to build the dependency tree visualization)
- cargo-deps (to build the dependency tree visualization)
- git
- gitui
- termux-api (if running on Android, for text-to-speech support)

## Getting started
It is quite straightforward when working on a computer (rust must be installed), just do this:
- In the root directory execute `cargo run --release`
- Or use the "pglowrpg.sh" shell script which does the same

You must run pGLOWrpg from the folder that contains "options", "presets", "locale" and "save"
folders, so there is no need to do "cargo install".

[splash]: doc/images/pglowrpg_banner.png "Banner"
[ui_sample]: doc/images/pglowrpg_ui_sample.jpg "Ui sample"
[biomes]: doc/images/biomes_example.png "Biomes map of some medium continent"
[animation]: doc/images/pglowrpg.gif "Animation"