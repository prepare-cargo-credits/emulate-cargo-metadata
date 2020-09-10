#[macro_use]
extern crate clap;

use cargo::util::Config;
use cargo::core::shell::Shell;
use cargo::CliError;
use clap::{App, SubCommand, ArgMatches};
use cargo::util::command_prelude::ArgMatchesExt;
use cargo::ops::OutputMetadataOptions;


fn main() {
    let matches = App::new(crate_name!()).author(crate_authors!()).about(crate_description!()).version(crate_version!())
        .subcommand(SubCommand::with_name("sinofseven").about("test call"))
        .get_matches();

    let mut config = match Config::default() {
        Ok(config) => config,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    match matches.subcommand() {
        ("sinofseven", Some(arg)) => cli_main(arg, &mut config),
        _ => {
            eprintln!("error subcommand");
            Err(CliError::code(1))
        }
    };
}

fn cli_main(args: &ArgMatches, config: &mut Config) -> Result<(), CliError> {
    let ws = match args.workspace(config) {
        Ok(ws) => ws,
        Err(e) => return Err(CliError::from(e))
    };

    let option = OutputMetadataOptions {
        features: vec![],
        no_default_features: false,
        all_features: false,
        no_deps: false,
        version: 1,
        filter_platforms: vec![]
    };

    let result = cargo::ops::output_metadata(&ws, &option)?;
    let text = match serde_json::to_string_pretty(&result) {
        Ok(text) => text,
        Err(e) => return Err(CliError::code(1))
    };
    match std::fs::write("sample_metadata.json", &text) {
        Ok(_) => Ok(()),
        Err(e) => Err(CliError::code(1))
    }
}

