# ToastScript

A PostScript-inspired stack-based language bootstrapped with Rust

## Usage:

`{toastCompiler} sourceFile [compilerArgs]`

- Compile using the specified compiler
  - toastCompiler: The toast compiler command to use
  - sourceFile: The file to compile
  - compilerArgs: Any arguments to pass trough to the compiler

## Compilers
### Bootstrap
`cargo run sourceFile [compilerArgs]`

- Run toast using the Rust compiler
  - Example:
  - `cargo run sourceFile [compilerArgs]`

### Self-Hosted(Not-yet implemented)[Not even close]:
`toast`
- The command to run compile using the self-hosted compiler
  - Example:
  - `toast sourceFile [compilerArgs]`

## Deja Vu?
You might have heard of the TypeScript implementation of toast. While not in development any longer, you can find it [here](https://github.com/MMukundi/toast-ts) 
