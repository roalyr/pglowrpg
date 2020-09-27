# pGLOWrpg - Procedurally generated living open worlds RPG
![Banner][splash]

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

## The game itself
![Biomes map of some medium continent][biomes]
Currently, pGLOWrpg's only feature is general world generation with
image maps output (both colorized and raw, if you want to use them for something else).
Feel free to play with world presets to find out interesting options combinations.
The ***presets_user*** folder is ignored by git, so store your custom presets there.

[splash]: doc/images/pglowrpg_banner.png "Banner"
[biomes]: doc/images/biomes_example.png "Biomes map of some medium continent"
