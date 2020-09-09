# pGLOWrpg
Procedurally generated living open worlds RPG

## Requirements
- rust

### Optional requirements (for the "common actions" wrapper)
- python3
- graphviz (to build the dependency tree visualization)
- cargo-deps (to build the dependency tree visualization)
- git
- gitui
- termux-api (if running on Android, for text-to-speech support)

## Getting started
It is quite straightforward when working on a computer (rust must be installed), just do this:
- cargo build --release
- cargo run --release
Running pGLOWrpg must be from withing the root directory that contains
"options", "presets", "locale" and "save folder", so there is no need to "cargo install".