use clap::Parser;
use log::{debug, info, warn};

mod ast;
mod codegen;
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
    parser.parse();
    let code_generator = codegen::CodeGenerator {};
    code_generator.generate();
}
