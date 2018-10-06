extern crate clap;

use std::io;
use std::env;
use std::path;
use std::ffi;

pub fn get_args<'a>() -> clap::ArgMatches<'a> {
    let name_arg = clap::Arg::with_name("name")
        .short("n")
        .long("name")
        .value_name("NAME")
        .help("set the junit suite name. This is also the file name");


    clap::App::new("test junit")
        .about("Creates junit XML from cargo-test output")
        .bin_name("cargo")
        .subcommand(clap::SubCommand::with_name("test-junit")
            .about("Converts cargo test output into a junit report")
            .arg(name_arg)
            .arg(clap::Arg::with_name("features")
                .long("features")
                .value_name("FEATURES"))
            .arg(clap::Arg::with_name("target-dir")
                .long("target-dir")
                .value_name("TARGET-DIR")))
        .get_matches()
}

pub fn get_file_name(matches: &clap::ArgMatches) -> io::Result<String> {
    let sub_match = matches.subcommand_matches("test-junit")
        .unwrap();

    sub_match.value_of("name")
        .map(str::to_string)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Name arg not provided"))
        .or_else(|_| env::current_dir().and_then(get_last_path_part))
}

fn get_last_path_part(p: path::PathBuf) -> io::Result<String> {
    p.iter()
        .last()
        .and_then(ffi::OsStr::to_str)
        .map(str::to_string)
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "Could not parse current dir"))
}
