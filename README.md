# `lightsaber` - A Rust toolkit

## Description

Lightsaber is a simple Rust toolkit for bootstrapping a new Rust project. It includes:

By default, enhanced runtime error handling (`eyre`) and logging (`tracing`) is included. 

For project-specific dependencies, `Cargo.toml` has a list of popular packages to use depending on the use case.

- 📦 Always nice-to-have and project-specific packages
- 🚨 Opinionated and strict `clippy` linting

## Lint in `watch` mode

```sh
# Install background check
cargo install bacon

# Lint and watch
bacon clippy
```

