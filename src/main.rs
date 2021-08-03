// SPDX-License-Identifier: Apache-2.0

//! The Enarx Keep runtime binary.
//!
//! It can be used to run a Wasm file with given command-line
//! arguments and environment variables.
//!
//! ## Example invocation
//!
//! ```console
//! $ wat2wasm fixtures/return_1.wat
//! $ RUST_LOG=enarx_wasmldr=info RUST_BACKTRACE=1 cargo run return_1.wasm
//!     Finished dev [unoptimized + debuginfo] target(s) in 0.07s
//!      Running `target/x86_64-unknown-linux-musl/debug/enarx-wasmldr target/x86_64-unknown-linux-musl/debug/build/enarx-wasmldr-c374d181f6abdda0/out/fixtures/return_1.wasm`
//! [2020-09-10T17:56:18Z INFO  enarx_wasmldr] got result: [
//!         I32(
//!             1,
//!         ),
//!     ]
//! ```
//!
//! On Unix platforms, the command can also read the workload from the
//! file descriptor (3):
//! ```console
//! $ RUST_LOG=enarx_wasmldr=info RUST_BACKTRACE=1 cargo run 3< return_1.wasm
//! ```
//!
#![deny(missing_docs)]
#![deny(clippy::all)]

mod cli;
mod workload;

use clap::crate_version;

use log::{debug, info};

use std::fs::File;
use std::io::Read;

fn main() {
    // Initialize the logger, taking settings from the default env vars
    env_logger::Builder::from_default_env().init();

    info!("version {} starting up", crate_version!());

    debug!("parsing argv");
    let args = cli::parse_args();
    debug!("module: {:?}", args.value_of("MODULE"));
    debug!("args: {:?}", args.value_of("ARGS"));

    let mut reader = if let Some(path) = args.value_of("MODULE") {
        info!("reading {}", path);
        File::open(&path).expect("Unable to open file")
    } else {
        unreachable!(); // Required args can't be missing...
    };

    let mut bytes = Vec::new();
    reader
        .read_to_end(&mut bytes)
        .expect("Failed to load workload");

    let wasm_args: Vec<String> = if let Some(a) = args.values_of("WASM_ARGS") {
        a.map(|s| s.to_owned()).collect()
    } else {
        Vec::new()
    };

    let wasm_env: Vec<(String, String)> = std::env::vars().collect();

    // FUTURE: measure wasm_env and wasm_args

    let result = workload::run(bytes, &wasm_args, &wasm_env).expect("Failed to run workload");

    info!("got result: {:#?}", result);
}
