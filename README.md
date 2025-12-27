# Neix - ⚡ Blazing Fast Eix-like Search for Nixpkgs

Neix is a fast, efficient search tool for [nixpkgs](https://github.com/NixOS/nixpkgs), inspired by [`eix`](https://github.com/vaeth/eix). It helps you quickly find packages and their metadata in the Nix ecosystem.

## ✨ Features
- Simple CLI
- Fast, indexed search for nixpkgs packages
- Lightweight and written in Rust

## 📦 Usage
```text
Usage: neix [OPTIONS] [QUERY]

Arguments:
  [QUERY]

Options:
  --update
  -l, --limit <LIMIT>  [default: 10]
  -h, --help           Print help
  -V, --version        Print version
```

## 🔍 Note
Neix searches through **your local nixpkgs** and outputs packages matching your architecture. If you need to search through all architectures, please [submit an issue](link-to-issues).


## 🤔 Why?
Because `nix search` and evaluating packages can be slow. Neix provides **instant results** with short, intuitive commands.

## 🛠️ Contributing
If you have a feature request, bug report, or want to contribute, feel free to:
- Submit a [pull request](link-to-pull-requests)
- Open an [issue](link-to-issues)

## 📥 Installation

### Try it

```sh
nix run github:Hovirix/neix -- rust --update
````

### Add to your `flake.nix`

```nix
neix.url = "github:Hovirix/neix";
```

Then reference the package in any Nix file:

```nix
neix.packages.${pkgs.system}.default
```

### With `nix profile`

```sh
nix profile add github:Hovirix/neix#default
```
