# Qwydy

**Qwydy** is an experimental programming language based on Iron Ossetic.

The project is currently in an early development stage. It includes a lexer, a parser, and a basic interpreter written in Rust.

## Features

- Ossetian-inspired keywords
- Variables and arithmetic expressions
- Conditional statements
- Basic function declarations and calls
- Unicode identifiers
- Lexer tests

## Example

```text
функци сӕйраг() {
    ивгӕ нымӕц = 42;
    кӕд нымӕц == 42 {
        ивгӕ дзуапп = нымӕц + 8;
        мыхуыр(дзуапп);
    } æндæра {
        мыхуыр(0);
    }
}
```

Expected output:

```text
50
```

## Build and run

Install Rust, then run:

```powershell
cargo +stable-gnu test
cargo +stable-gnu run -- hello.qw
```

To inspect tokens:

```powershell
cargo +stable-gnu run -- --tokens first.qw
```

## Status

Qwydy is experimental and under active development. The current implementation is an interpreter, not yet an LLVM-based native compiler.

## License

No license has been selected yet.