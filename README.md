# Basic tree-sitter + cranelift jit

A basic working example of tree-sitter + cranelift jit.

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

If you modify the grammar, `cargo build` in the main project will update everything as needed.

The main binary uses syntax highlighting as an example of that.