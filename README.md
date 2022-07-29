# Lightsaber: A Rust toolkit

## Description

Lightsaber is a Rust toolkit for bootstrapping a new Rust project. It includes:

- 📦 Packages that are always nice-to-have and project-specific packages. For
  more information, see [`Cargo.toml`](./Cargo.toml)
- 🚨 Opinionated and strict `clippy` linting

## A usual workflow

```sh
# Ensure you have installed background check tool
cargo install bacon

# Lint and watch whilst developing
bacon clippy
```

## Library or binary 

Depending on the usecase, you'll likely develop either a binary or a library.

If what you are creating will be an executable, you'll need to delete [`src/main.rs`](/src/main.rs).
If what you are creating will be a library, you'll need to delete [`src/lib.rs`](/src/lib.rs).

Make sure that the `clippy` configuration is added in the base file that remains.

