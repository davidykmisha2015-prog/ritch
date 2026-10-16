# ritch

![ritch screenshot](image.png)

A fast and minimalist terminal utility written in Rust with zero dependencies.

## Installation

### NixOS (via Flakes)
1. Add `ritch` to your `flake.nix` inputs:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    ritch.url = "github:davidykmisha2015-prog/ritch";
  };
}
```

2. Add the package to your `environment.systemPackages` (e.g., in `configuration.nix`):

```nix
environment.systemPackages = [
  inputs.ritch.packages.${pkgs.system}.default
];
```

---
#### Or run it directly without installation:
```bash
nix run github:davidykmisha2015-prog/ritch
```

### Other Linux Distros (via Cargo)
Build and install directly from the repository:
```bash
cargo install --git https://github.com/davidykmisha2015-prog/ritch
```

> 💡 **Note:** Make sure you have `cargo` installed on your system!

## Usage

Run `ritch` to automatically detect your OS, or preview a specific distro banner (`nixos`, `arch`, `debian`, `fedora`, `void`):
```bash
ritch --logo arch
```

---
> 💡 **Easter Egg:** There is one secret rainbow banner hidden inside `src/main.rs` that isn't a Linux distro at all.

## License
This project is licensed under the MIT License.
