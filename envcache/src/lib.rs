//! # 🏴‍☠️ envcache
//! The envcache crate (**env**ironment **cache**) lets users cache
//! environmental variables in a build.rs script, so that subsequent calls to
//! cargo are not required to be ran with the same variable specified.
//!
//! For example, let's assume you have a build.rs that **requires** `SOME_VAR`
//! to be specified for work. Maybe its a path to some library that lives
//! outside of the Rust ecosystem (like LLVM):
//!
//! ```sh
//! SOME_VAR=42 cargo test
//! cargo clippy # Will fail because SOME_VAR is not set
//! ```
//!
//! This would fail because the run to `cargo clippy` requires that it is run
//! with `SOME_VAR=42`. With the envcache, we can use a `build.rs` that ensures
//! this will run:
//!
//! ```
//! use envcache;
//! # std::env::set_var("OUT_DIR", std::env::temp_dir());
//! let mut envcache = envcache::EnvCache::new();
//! envcache.cache("SOME_VAR");
//! ```
//!
//! Now if we run this again:
//!
//! ```sh
//! SOME_VAR=42 cargo test
//! cargo clippy # SOME_VAR will be 42
//! ```
//!
//! You can change a previously set cached variable by simply re-specifying it
//! on the command line:
//!
//! ```sh
//! SOME_VAR=42 cargo test
//! SOME_VAR=13 cargo test
//! cargo test # SOME_VAR will be 13!
//! ```
//!
//! Note that running `cargo clean` will remove any previously cached variables,
//! so running:
//!
//! ```sh
//! SOME_VAR=42 cargo test
//! cargo clippy # Will work because we've cached SOME_VAR
//! cargo clean
//! cargo test # Will fail because SOME_VAR won't be set
//! ```

use std::collections::HashMap;
use std::env;
use std::fs::*;
use std::io::{BufReader, Read, Write};
use std::path::*;

pub struct EnvCache {
    entries: HashMap<String, String>,
}

fn cache_path() -> PathBuf {
    Path::new(&env::var("OUT_DIR").unwrap()).join("envcache.config")
}

impl EnvCache {
    /// Create a new `EnvCache`. This only works inside a `build.rs` with
    /// `OUT_DIR` specified.
    pub fn new() -> Self {
        let cache_path = cache_path();

        let mut entries = HashMap::new();

        if let Ok(file) = File::open(cache_path) {
            let mut f = BufReader::new(file);
            let mut string = String::new();
            f.read_to_string(&mut string).unwrap();

            let mut index = 0;

            while index < string.len() {
                let env_len = string[index..index + 8].parse::<u32>().unwrap() as usize;
                index += 8;

                let env = &string[index..index + env_len];
                index += env_len;

                let val_len = string[index..index + 8].parse::<u32>().unwrap() as usize;
                index += 8;

                let val = &string[index..index + val_len];
                index += val_len;

                entries.insert(env.to_string(), val.to_string());
            }
        }

        Self { entries }
    }

    /// Cache a variable `env` into the envcache.
    ///
    /// Returns the value of the environment variable.
    pub fn cache<'a>(&'a mut self, env: &str) -> Option<&'a str> {
        if let Ok(var) = env::var(env) {
            self.entries.insert(env.to_string(), var);
        }

        if let Some(var) = self.entries.get(&env.to_string()) {
            Some(var)
        } else {
            None
        }
    }
}

impl Default for EnvCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EnvCache {
    fn drop(&mut self) {
        let cache_path = cache_path();

        if let Ok(mut file) = File::create(cache_path) {
            for (key, val) in self.entries.iter() {
                println!("cargo:rerun-if-env-changed={}", key);
                println!("cargo:rustc-env={}={}", key, val);
                write!(file, "{:08x}{}{:08x}{}", key.len(), key, val.len(), val).unwrap();
            }
        }
    }
}
