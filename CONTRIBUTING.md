# Compile and run the code

vkmsctl is written in Rust and it can be built using cargo:

```bash
$ cargo build --features="vkmsctl-cli"
```

To run the vkmsctl command line tool:

```bash
$ cargo run --features="vkmsctl-cli" -- <CLI arguments>
```


# Architecture

This project is organized into two key components: the vkmsctl command line
interface tool and a library containing its logic that can be reused by third
party projects.

## vkmsctl CLI (`src/main.rs`)

The vkmsctl binary provides a CLI interface for interacting with the
[VKMS (Virtual Kernel Modesetting)](https://docs.kernel.org/gpu/vkms.html)
configfs configuration system. For more information about the available options:

```bash
$ cargo run --features="vkmsctl-cli" -- --help
```

## vkmsctl Library (`src/lib.rs`)

The heart of the project is a library that provides abstractions for interacting
with the VKMS configfs API.


# Development environment

You can use any text editor or IDE. But, if like me, you are using Visual Studio
Code, the recommended extensions to install are:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer):
  Rust language support with code completion, diagnostics, refactoring and more
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb):
  Native debugger for Rust (used in the debug configuration)
- [EditorConfig](https://marketplace.visualstudio.com/items?itemName=EditorConfig.EditorConfig):
  Configure vscode using the .editorconfig file
- [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker):
  Install this extension to avoid typos

For more information about how to setup the development environment, check the
[official documentation](https://code.visualstudio.com/docs/languages/rust).


# Contact the developer

If you want to report a bug or ask a question, you can do it in the official bug
tracker:

https://github.com/JoseExposito/vkmsctl/issues

Happy coding! ❤️
