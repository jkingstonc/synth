use std::time::Instant;

use clap::Parser;
use log::{debug, info, warn};

mod ast;
mod codegen;
mod ir;
mod ir_parse;
mod lex;
mod parse;
mod types;

const VERSION: &str = "0.0.1";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to run
    #[arg(short, long)]
    file: String,
}

fn main() {
    let now = Instant::now();
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("synth {}", VERSION);

    let args = Args::parse();

    let source = std::fs::read_to_string(args.file).expect("unable to read source file test.trove");

    let mut lexer = lex::Lexer::new();
    lexer.lex(Box::new(source));

    for token in lexer.tokens.iter() {
        debug!("token {:?}.", token);
    }

    let mut parser = parse::Parser {
        tokens: &lexer.tokens,
    };
    let ast = parser.parse();
    debug!("ast {:?}", ast);

    let mut ir_parser = ir_parse::IRParser {};
    let ir = ir_parser.parse();
    debug!("ir {:?}", ir);

    let code_generator = codegen::X86CodeGenerator { ir };
    code_generator.generate();
    let elapsed = now.elapsed();
    debug!(
        "compilation time elapsed {:.2?}ms ({:.2?}s).",
        elapsed.as_millis(),
        elapsed.as_secs()
    );
}
