use cargo::util::Config;
use cargo::core::shell::Shell;
use cargo::CliError;
use clap::{App, SubCommand, ArgMatches};
use cargo::util::command_prelude::ArgMatchesExt;
use cargo::ops::OutputMetadataOptions;


fn main() {
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };

    let result = match cargo::ops::fix_maybe_exec_rustc() {
        Ok(true) => Ok(()),
        Ok(false) => {
            let _token = cargo::util::job::setup();
            cli_main(&mut config)
        },
        Err(e) => Err(CliError::from(e))
    };

    match result {
        Err(e) => cargo::exit_with_error(e, &mut *config.shell()),
        Ok(()) => {}
    }
}

fn cli_main(config: &mut Config) -> Result<(), CliError> {
    let app = SubCommand::with_name("metadata").get_matches();

    let (_, args) = app.subcommand();
    // let args = args.unwrap();
    let args = ArgMatches::default();

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

