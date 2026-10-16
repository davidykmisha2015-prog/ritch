# ritch

![ritch screenshot](image.png)

A fast and minimalist terminal utility written in Rust with zero dependencies.

## Installation

### Nix / NixOS (Recommended)
Run directly via Nix Flakes without manual installation:
```bash
nix run github:davidykmisha2015-prog/ritch
```

### Other Linux Distros (via Cargo)
Build and install directly from the repository:
```bash
cargo install --git [https://github.com/davidykmisha2015-prog/ritch](https://github.com/davidykmisha2015-prog/ritch)
```
---
💡 **Cargo:** Make sure you installed cargo in your system!

## Usage

Run `ritch` to automatically detect your OS, or preview a specific distro banner (`nixos`, `arch`, `debian`, `fedora`, `void`):
```bash
ritch --logo arch
```

---
> 💡 **Easter Egg:** There is one secret rainbow banner hidden inside `src/main.rs` that isn't a Linux distro at all. Can you defy *gravity* and find the `--logo` name for it?

This project use MIT license
