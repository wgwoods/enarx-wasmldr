// SPDX-License-Identifier: Apache-2.0

#![allow(missing_docs, unused_variables)] // This is a work-in-progress, so...

use structopt::{StructOpt, clap::AppSettings};

use std::path::PathBuf;
use anyhow::{bail, Result};

// The main StructOpt for running `wasmldr` directly
#[derive(StructOpt, Debug)]
#[structopt(setting=AppSettings::TrailingVarArg)]
pub struct RunOptions {
    /// Pass an environment variable to the program
    #[structopt(
        short = "e",
        long = "env",
        number_of_values = 1,
        value_name = "NAME=VAL",
        parse(try_from_str=parse_env_var),
    )]
    pub envs: Vec<(String, String)>,

    /// Name of the function to invoke
    #[structopt(long, value_name = "FUNCTION")]
    invoke: Option<String>,

    #[structopt(flatten)]
    wasmtime: WasmtimeOptions,

    // TODO: --inherit-env
    // TODO: --stdin, --stdout, --stderr

    /// Path of the WebAssembly module to run
    #[structopt(
        index = 1,
        required = true,
        value_name = "MODULE",
        parse(from_os_str),
    )]
    pub module: PathBuf,

    // NOTE: this has to come last for TrailingVarArg
    /// Arguments to pass to the WebAssembly module
    #[structopt(value_name="ARGS")]
    pub args: Vec<String>,
}

// Options that change the behavior of wasmtime
#[derive(StructOpt, Debug)]
struct WasmtimeOptions {
    /// Enable or disable WebAssembly features
    #[structopt(long, value_name = "FEATURE,FEATURE,...", parse(try_from_str = parse_wasm_features))]
    wasm_features: Option<wasmparser::WasmFeatures>,
}

fn parse_env_var(s: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        bail!("must be of the form `NAME=VAL`");
    }
    Ok((parts[0].to_owned(), parts[1].to_owned()))
}

const SUPPORTED_WASM_FEATURES: &[(&str, &str)] = &[
    ("all", "enable all supported WebAssembly features"),
    ("module-linking", "enable support for the module-linking proposal (experimental; implies multi-memory)"),
    ("multi-memory", "enable support for the multi-memory proposal (experimental)"),
];

fn parse_wasm_features(s: &str) -> Result<wasmparser::WasmFeatures> {
    let mut features = wasmparser::WasmFeatures::default();
    let s = s.trim();
    /* TODO: match against SUPPORTED_WASM_FEATURES */
    match s {
        "module-linking" => features.module_linking = true,
        _ => bail!("unknown feature {:?}", s)
    }
    Ok(features)
}