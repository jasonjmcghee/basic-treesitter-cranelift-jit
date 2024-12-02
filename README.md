# Basic tree-sitter + cranelift jit

A basic working example of tree-sitter + cranelift jit.

There's really one core file - I could break it up, but wanted it to be in one place for this basic example.

[The one core file](src/language/mod.rs)

Here's the [grammar](./tree-sitter-calculator/grammar.js).

## Video

This is a bit outdated - was taken before divide was added!

https://github.com/user-attachments/assets/6768b986-63d9-430f-bdec-943869b96b95

## Testing things
You can run the repl (using crossterm)

```bash
cargo run --release
```

Or do a little stress test (using cross term)

```bash
cargo run --example stress
```

## Benchmarks

Also some simple benchmarks.

Very similar to the "stress" test

```bash
 cargo bench --bench calculator_random_bench
 ```

And a more basic, non-random one

```bash
 cargo bench --bench calculator_bench
```

## Modifying things

If you modify the [grammar](./tree-sitter-calculator/grammar.js), `cargo build` in the main project will update everything as needed.

The main binary uses syntax highlighting as an example of that.
