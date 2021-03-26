# 🏴‍☠️ envcache

[![Actions Status](https://github.com/sheredom/envcache/workflows/Rust/badge.svg)](https://github.com/sheredom/envcache/actions)
[![Crates.io](https://img.shields.io/crates/v/envcache.svg)](https://crates.io/crates/envcache)
[![API Docs](https://docs.rs/envcache/badge.svg)](https://docs.rs/envcache)

The envcache crate (**env**ironment **cache**) lets users cache
environmental variables in a build.rs script, so that subsequent calls to
cargo are not required to be ran with the same variable specified.

For example, let's assume you have a build.rs that **requires** `SOME_VAR`
to be specified for work. Maybe its a path to some library that lives
outside of the Rust ecosystem (like LLVM):

```sh
SOME_VAR=42 cargo test
cargo clippy # Will fail because SOME_VAR is not set
```

This would fail because the run to `cargo clippy` requires that it is run
with `SOME_VAR=42`. With the envcache, we can use a `build.rs` that ensures
this will run:

```
use envcache;
# std::env::set_var("OUT_DIR", std::env::temp_dir());
let mut envcache = envcache::EnvCache::new();
envcache.cache("SOME_VAR");
```

Now if we run this again:

```sh
SOME_VAR=42 cargo test
cargo clippy # SOME_VAR will be 42
```

You can change a previously set cached variable by simply re-specifying it
on the command line:

```sh
SOME_VAR=42 cargo test
SOME_VAR=13 cargo test
cargo test # SOME_VAR will be 13!
```

Note that running `cargo clean` will remove any previously cached variables,
so running:

```sh
SOME_VAR=42 cargo test
cargo clippy # Will work because we've cached SOME_VAR
cargo clean
cargo test # Will fail because SOME_VAR won't be set
```
