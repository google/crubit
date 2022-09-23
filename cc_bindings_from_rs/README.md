# cc_bindings_from_rs

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.


## Invoking as a standalone executable

Most `rustc` cmdline parameters should be supported (e.g. `--crate-type`).

Example:

```
# Set CARGO_TARGET_DIR to avoid generating build artifacts inside of the source
# tree (i.e. generate them elsewhere - where `hg status` cannot see them).
$ export CARGO_TARGET_DIR=$HOME/scratch/cargo-target

$ cat $HOME/scratch/test.rs
pub fn public_function() {
    private_function()
}

fn private_function() {}

$ cargo run -- \
    --h_out=$HOME/scratch/test.h -- \
    $HOME/scratch/test.rs --crate-type=lib --sysroot `rustc --print sysroot`

$ cat $HOME/scratch/test.h
// List of public functions:
// public_function
```


## Contributing

See
[rs_bindings_from_cc/README.md](../rs_bindings_from_cc/README.md#contributing).
