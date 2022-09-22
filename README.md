# ToastScript

A PostScript-inspired stack-based language bootstrapped with Rust

## Examples
[Example directory](./examples/)

## Usage:

`{toast_compiler} [options] source_file`

- Compile using the specified compiler
  - toast_compiler: The toast compiler command to use
  - source: The file to compile
  - options: Any arguments to pass through to the compiler

## Compilers
### Bootstrap
`cargo run [options] source_file`
- Run toast using the Rust compiler
### Self-Hosted [^1] (Not-yet implemented)[Not even close]:

`toast [options] source_file`
- The command to run compile using the self-hosted compiler


## Deja Vu?
You might have heard of the TypeScript implementation of toast. While not in development any longer, you can find it [here](https://github.com/MMukundi/toast-ts) 

[^1]: [src](https://en.wikipedia.org/wiki/Self-hosting_(compilers)) Self-hosted *(adjective)* *(Of a program)* To be used as part of a toolchain to output new versions of that same program.
    - *(Of a compiler)* To be written in the language it compiles.
