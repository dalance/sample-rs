# sample-rs

This is a sample project of [Rust](https://www.rust-lang.org/) for tring full project layout.

## Project layout

The full project layout is the following.
Please see [the official document](http://doc.crates.io/manifest.html#the-project-layout) for the details.

```
▾ src/           # directory containing source files
  lib.rs         # the main entry point for libraries and packages
  main.rs        # the main entry point for projects producing executables
  ▾ bin/         # (optional) directory containing additional executables
    *.rs
▾ examples/      # (optional) examples
  *.rs
▾ tests/         # (optional) integration tests
  *.rs
▾ benches/       # (optional) benchmarks
  *.rs
```

## Available `cargo` command

The following commands can be tried.

- `cargo build`
- `cargo run --bin hello`
- `cargo run --bin world`
- `cargo run --example hello_world`
- `cargo test`
- `cargo +nightly bench`

`cargo bench` requires nightly Rust, so you should use `+nightly` option.

