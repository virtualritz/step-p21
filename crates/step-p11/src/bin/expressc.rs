//! Executable for step_p11 EXPRESS language compiler

use clap::Parser;
use std::{fs, path::*};
use step_p11::{ast::SyntaxTree, codegen::rust::*, ir::IR};

/// Compile an EXPRESS schema into Rust.
#[derive(Debug, Parser)]
#[command(about, version)]
struct Arguments {
    /// How many lines of the offending source to print per syntax error.
    #[arg(long = "num-error-lines", default_value_t = 10)]
    number_of_error_lines: usize,
    /// Check that the input EXPRESS definitions parse, then stop.
    #[arg(long)]
    check: bool,
    /// The EXPRESS schema to compile.
    source: PathBuf,
}

fn main() {
    let args = Arguments::parse();
    let src = fs::read_to_string(&args.source)
        .expect("Failed to load EXPRESS source code");
    let st = match SyntaxTree::parse(&src) {
        Ok(st) => st,
        Err(e) => {
            for (code, kind) in e.errors {
                eprintln!(
                    "Syntax Error occurred while parsing following line [{:?}]:",
                    kind
                );
                for line in code.lines().take(args.number_of_error_lines) {
                    eprintln!("> {}", line);
                }
                eprintln!();
            }
            panic!("Syntax Error");
        }
    };
    if args.check {
        eprintln!("Parse succeeded");
        return;
    }

    let ir =
        IR::from_syntax_tree(&st).expect("Failed in semantic analysis phase");
    println!(
        "#![allow(dead_code)]\n{}",
        ir.to_token_stream(CratePrefix::Internal)
    );
}
