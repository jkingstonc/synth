use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    io::{self, BufWriter},
    time::Instant,
};

use clap::Parser;
use log::{debug, error, info};

use crate::{
    codegen::llvm::LLVMCodeGenerator,
    codegen::x86::X86CodeGenerator,
    compiler::CompilerOptions,
    optimize::{GeneralPassIROptimizer, IROptimizer},
};

mod ast;
mod codegen;
mod compiler;
mod comptime;
mod ir;
mod ir_interpret;
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
    // whether to interpret
    #[arg(short, long)]
    interpret: Option<bool>,

    /// Name of the file to run
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    arch: String,
    #[arg(short, long)]
    write_ir: Option<bool>,
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

    if args.interpret.expect("interpret") {
        while true {
            print!(">");
            io::stdout().flush();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut optimization: usize = 0;
            if let Some(o) = args.optimize {
                optimization = o;
            }
            let compiler_options = CompilerOptions {
                optimization,
                current_file: "<interpret>".to_string(),
            };

            let mut lexer = lex::Lexer::new();
            lexer.lex(Box::new(line));

            let mut parser = parse::Parser {
                tokens: &lexer.tokens,
            };
            let ast = parser.parse();

            let mut ir_parser = ir_parse::IRParser {
                compiler_options: &compiler_options,
                counter: 0,
                block_counter: 0,
                locals_counter: 0,
            };
            let mut main_block = ir_parser.parse(ast);
            let mut ir_interpreter = ir_interpret::IRInterpreter {
                compiler_options: &compiler_options,
                counter: 0,
                variables_map: HashMap::new(),
            };
            let result = ir_interpreter.execute(&main_block);
            println!("{:?}", result);
        }
        return;
    }

    let mut optimization: usize = 0;
    if let Some(o) = args.optimize {
        optimization = o;
    }
    let compiler_options = CompilerOptions {
        optimization,
        current_file: args.file.to_string(),
    };

    let source = std::fs::read_to_string(args.file.to_string())
        .expect("unable to read source file test.trove");

    let mut lexer = lex::Lexer::new();
    lexer.lex(Box::new(source));

    let mut parser = parse::Parser {
        tokens: &lexer.tokens,
    };
    let ast = parser.parse();

    let mut ir_parser = ir_parse::IRParser {
        compiler_options: &compiler_options,
        counter: 0,
        block_counter: 0,
        locals_counter: 0,
    };
    let mut main_block = ir_parser.parse(ast);
    if let Some(write_ir) = args.write_ir {
        if write_ir {
            let f: File = File::create("./build/build.sir").expect("unable to create file");
            let mut writer = BufWriter::new(f);
            // for instruction in instructions.iter() {
            //     write!(writer, "{}\n", instruction.to_string_for_writing())
            //         .expect("unable to write");
            // }
            write!(writer, "{}\n", main_block.to_string_for_writing()).expect("unable to write");
        }
    }

    // let mut comptime_analyzer = comptime::ComptimeAnalyzer { ir: instructions };
    // let mut instructions = comptime_analyzer.analyze();

    // match args.optimize {
    //     Some(1) => {
    //         // some optimization
    //         let mut general_pass_ir_optimizer = GeneralPassIROptimizer { ir: instructions };
    //         instructions = general_pass_ir_optimizer.optimize();
    //     }
    //     None => {}
    //     _ => {}
    // }

    // // tmp
    // let mut ir_interpreter = ir_interpret::IRInterpreter {
    //     compiler_options: &compiler_options,
    //     counter: 0,
    //     variables_map: HashMap::new(),
    // };
    // ir_interpreter.execute(&main_block);

    // optimization stage

    match args.arch.as_str() {
        "x86" => {
            let mut code_generator = LLVMCodeGenerator {
                str_buffer: "".to_string(),
            };
            code_generator.generate(&main_block);
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
