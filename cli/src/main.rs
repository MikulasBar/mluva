use clap::Parser;

use crate::{cli::Cli, commands::Commands};
mod cli;
mod commands;
mod config;
mod module_metadata;

use std::error::Error;
use std::fs;

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{Config, DisplayStyle, emit_to_io_write};

fn run_tokenize_test(path: &str) -> Result<(), Box<dyn Error>> {
    // read source
    let src = fs::read_to_string(path)?;

    // prepare files map for codespan-reporting
    let mut files = SimpleFiles::<String, String>::new();
    let file_id = files.add(path.to_string(), src.clone());

    // call library lexer that returns Token or CompileError
    match mluva::compiler::tokenize(file_id, &src) {
        Ok(tokens) => {
            println!("OK: tokenized {} tokens", tokens.len());
            Ok(())
        }
        Err(compile_error) => {
            // convert to a codespan Diagnostic and print nicely to stderr
            let diag = compile_error.to_diagnostic(&files);
            let writer = StandardStream::stderr(ColorChoice::Auto);
            let mut config = Config::default();
            emit_to_io_write(&mut writer.lock(), &Config::default(), &files, &diag)?;
            // return an error for non-zero exit code
            Err("tokenization failed".into())
        }
    }
}

fn main() {
    // let cli = Cli::parse();

    // match cli.command {
    //     Commands::Init => {
    //         if let Err(e) = commands::init::command() {
    //             eprintln!("Error during init: {}", e);
    //         }
    //     }

    //     Commands::Run => {
    //         let res = commands::run::command();
    //         if let Err(e) = res {
    //             eprintln!("{}", e);
    //         }
    //     }

    //     Commands::Uninit => {
    //         if let Err(e) = commands::uninit::command() {
    //             eprintln!("Error during uninit: {}", e);
    //         }
    //     }

    //     Commands::Build => {
    //         let res = commands::build::command();
    //         if let Err(e) = res {
    //             eprintln!("{}", e);
    //         }
    //     }
    // }

    // default file or first arg
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "test.mv".to_string());

    run_tokenize_test(&path);
}
