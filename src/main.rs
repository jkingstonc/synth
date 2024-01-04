use std::time::Instant;

use clap::Parser;
use log::{debug, error, info};

use crate::optimize::{GeneralPassIROptimizer, IROptimizer};

mod ast;
mod codegen;
mod comptime;
mod ir;
mod ir_parse;
mod lex;
mod optimize;
mod parse;
mod token;
mod types;

const VERSION: &str = "0.0.1";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to run
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    arch: String,
    #[arg(short, long)]
    optimize: Option<usize>,
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

    // for token in lexer.tokens.iter() {
    // debug!("token {:?}.", token);
    // }

    let mut parser = parse::Parser {
        tokens: &lexer.tokens,
    };
    let ast = parser.parse();
    // debug!("ast {:?}", ast);

    let mut ir_parser = ir_parse::IRParser {};
    let mut instructions = ir_parser.parse(ast);
    for instruction in instructions.iter() {
        debug!("instruction {:?}.", instruction);
    }

    let mut comptime_analyzer = comptime::ComptimeAnalyzer { ir: instructions };
    let mut instructions = comptime_analyzer.analyze();

    match args.optimize {
        Some(1) => {
            // some optimization
            let mut general_pass_ir_optimizer = GeneralPassIROptimizer { ir: instructions };
            instructions = general_pass_ir_optimizer.optimize();
        }
        None => {}
        _ => {}
    }

    // optimization stage

    match args.arch.as_str() {
        "x86" => {
            let code_generator = codegen::X86CodeGenerator { ir: instructions };
            code_generator.generate();
        }
        _ => {
            error!(
                "unsupported format {:?}, supported formats are [x86]",
                args.arch
            );
            return;
        }
    }
    let elapsed = now.elapsed();
    debug!(
        "compilation time elapsed {:.2?}ms ({:.2?}s).",
        elapsed.as_millis(),
        elapsed.as_secs()
    );
}
