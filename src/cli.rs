// SPDX-License-Identifier: Apache-2.0

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};

pub fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("enarx-wasmldr")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Sets up a measured runtime environment and runs a .wasm function")
        .arg(
            Arg::with_name("MODULE")
                .help("WebAssembly module to load")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FUNC")
                .help("Name of function to invoke")
                .long("--invoke")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ARGS")
                .help("Arguments to pass to the function")
                .multiple(true)
                .last(true),
        )
        .get_matches()
}
