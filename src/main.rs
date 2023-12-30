use clap::Parser;

mod lex;

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

    // println!("Welcome to synth");

    // let mut lexer = lex::Lexer::new();

    let args = Args::parse();

    let source = std::fs::read_to_string(args.file).expect("unable to read source file test.trove");

    println!("source:\n{}", source);
}
