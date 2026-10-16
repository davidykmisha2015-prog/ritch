# ritch

![ritch screenshot](image.png)

A fast and minimalist terminal system fetch utility written in Rust with zero external dependencies.

## Features

- Zero external crates (pure Rust standard library)
- Built-in ASCII logos for popular Linux distributions
- Displays OS and Linux kernel version with Nerd Font icons
- Declarative configuration in Nix format (`~/.config/ritch/config.nix`)

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

Or run directly without installation:

```bash
nix run github:davidykmisha2015-prog/ritch
```

### Other Linux Distros (via Cargo)

Build and install directly from the repository:

```bash
cargo install --git https://github.com/davidykmisha2015-prog/ritch
```

## Usage

Run with automatic distro detection:

```bash
ritch
```

Preview a specific distribution logo:

```bash
ritch --logo arch
```

Supported logos: `nixos`, `arch`, `debian`, `fedora`, `void`, `ubuntu`, `gentoo`, `mint` (`linuxmint`), `manjaro`, `alpine`.

> Note: There is also a secret rainbow banner hidden inside `src/main.rs`.

## Configuration

On the first run, `ritch` automatically generates a configuration file at `~/.config/ritch/config.nix`:

```nix
{
  logo = "auto";
  showOs = true;
  showKernel = true;
  osLabel = "os";
  kernelLabel = "kernel";
}
```

Removing `showOs` or `showKernel` from the file (or setting them to `false`) hides the corresponding line from the output.

## License

This project is licensed under the MIT License.
