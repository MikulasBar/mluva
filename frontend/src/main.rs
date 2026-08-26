use std::collections::HashMap;

use codespan_reporting::{
    files::SimpleFiles,
    term::{
        Config as CodespanConfig, emit_to_io_write,
        termcolor::{ColorChoice, StandardStream},
    },
};
use common::{CompileError, descriptor};
use common::{Descriptor, module::ModuleSigniture};
use compiler::Compiler;
use frontend::parse_source;
use typechecker::TypeChecker;

const SOURCE: &str = "

import mylib.mymodule

fn sum(x I32, y I32) I32 {
    return x + y
}

fn main() {
    let x I32 = 6565
    let y = 356

    x = mymodule.sum(x, y) + sum(x, 98)
}
";

fn process_result<T>(result: Result<T, CompileError>) -> T {
    result
        .map_err(|e| {
            let mut files = SimpleFiles::new();

            files.add("myfile.mv", SOURCE);
            let diag = e.to_diagnostic();
            let writer = StandardStream::stderr(ColorChoice::Auto);
            let Ok(_) = emit_to_io_write(
                &mut writer.lock(),
                &CodespanConfig::default(),
                &files,
                &diag,
            ) else {
                eprintln!("Failed to write diagnostics");
                panic!()
            };

            panic!("Something went wrong")
        })
        .ok()
        .unwrap()
}

fn build_deps() -> HashMap<Descriptor, ModuleSigniture> {
    let mut deps = HashMap::new();
    let descriptor = descriptor!("mylib", "mymodule");

    let mymodule_source = "
        fn sum(a I32, b I32) I32 {
            a + b
        }
    ";

    let ast = parse_source(mymodule_source, descriptor.clone(), 1).unwrap();

    deps.insert(descriptor, ast.to_signiture());

    deps
}

fn main() {
    println!("");
    let descriptor = descriptor!("main_module");
    let mut ast = process_result(parse_source(SOURCE, descriptor, 0));
    println!("frontend clean...");
    let dependencies = build_deps();

    process_result(TypeChecker::new(&mut ast, &dependencies).check());
    println!("typechecker clean...");
    let code = process_result(Compiler::new(&ast, &dependencies).compile());
    println!("compiler clean...");

    println!("CODE: {:#?}", code);
}
