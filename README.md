# Development

Project directory structure

```
project/
├─ assets/
├─ src/
│  ├─ main.rs 
│  ├─ views/ 
│  │  ├─ mod.rs 
│  │  ├─ home.rs 
├─ Cargo.toml
```

## Deps 

To run this project locally, install Rust and the dioxus-cli.

### Install Rust 

Visit https://rust-lang.org/learn/get-started to install Rust

### Install Dioxus CLI

Run `cargo install dioxus-cli`

## Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

