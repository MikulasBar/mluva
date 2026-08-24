use frontend::parse_source;

use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config as CodespanConfig, emit_to_io_write,
        termcolor::{ColorChoice, StandardStream},
    },
};

const SOURCE: &str = "

import mylib.mymodule

fn sum(x I32, y I32) I32 {
    return x + y
}

fn main() {
    let x I32 = 6565
    let y = 356

    x = sum(x, y)
}
";

fn main() {
    println!("sfsgs");
    let result = parse_source(SOURCE, 0);

    match result {
        Ok(ast) => {
            println!("AST: {:#?}", ast);
        },
        Err(e) => {
            let mut files = SimpleFiles::new();

            files.add("sfsg", SOURCE);
            let diag = e.to_diagnostic();
            let writer = StandardStream::stderr(ColorChoice::Auto);
            let Ok(_) = emit_to_io_write(
                &mut writer.lock(),
                &CodespanConfig::default(),
                &files,
                &diag,
            ) else {
                eprintln!("Failed to write diagnostics");
                panic!();
            };
        }
    }
}