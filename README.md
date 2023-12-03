# Advent of Code 2023

Some of it is made in Rust and some in Nushell


## Nushell

The scripts are a module, that you can just test using the test input:

```nushell
use std testing run-tests
run-tests
```
in the directory or just simply run on your data by:

```nushell
use part1.nu
part1.nu  
```
## Rust

The rust implementation is made as a workspace.
Each day has its own crate, that contains two binaries part1 and part2,
you can test the whole workspace by runing `cargo test` or specific day by running `cargo test  -p day-{n}`.

To run the solution simply run: `cargo run -p day-{n} --bin part{x}`
