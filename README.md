# pki-explorer

`pki-explorer` is a Rust TUI for browsing through collections of [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail)-encoded [X.509](https://en.wikipedia.org/wiki/X.509) TLS certificates.

The TUI will lookup files in a local directory, and print out informations about well-formed X.509 certificates in a orderly fashion, while ignoring other files. The lookup is recursive.

Simply clone this repository and run `pki-explorer` as a local Cargo create (`cargo run`). Use `[-h|--help]` to print CLI options, and `[-p|--path]` to use a different relative path. The default relative path is the local path (`.`).

*e.g.*
```
Usage: pki-explorer [OPTIONS]

Options:
  -p, --path <PATH>  [default: .]
  -s, --silent       [default: true]
  -h, --help         Print help
  -V, --version      Print version
```