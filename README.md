# pGLOWrpg
Procedurally generated living open worlds RPG

## Requirements
Main requirements:
- rust

Optional requirements (for the "common actions" wrapper):
- python3
- graphviz (to build the dependency tree visualization)
- cargo-deps (to build the dependency tree visualization)
- git
- gitui
- termux-api (if running on Android, for text-to-speech support)

## Getting started
It is quite straightforward when working on a computer (rust must be installed), just do this:
- cargo run --release
- Or use the "pglowrpg.sh" shell script which does the same

You must run pGLOWrpg from the folder that contains "options", "presets", "locale" and "save"
folders, so there is no need to do "cargo install".

Currently, pGLOWrpg's only feature is general world generation with
image maps output (both colorized and raw).
Feel free to play with world presets to find out interesting options combinations.
The **presets_user** folder is ignored by git, so store your custom presets there.
