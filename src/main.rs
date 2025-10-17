use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use thiserror::Error;

use ravensone::{compile_component, CompileError};

#[derive(Parser)]
#[command(author, version, about = "RavensOne compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a .raven file into TSX
    Build {
        /// Input .raven file
        input: PathBuf,
        /// Output path for generated TSX
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Compile(#[from] CompileError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        report_error(&err);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Commands::Build { input, output } => build(input, output),
    }
}

fn build(input: PathBuf, output: Option<PathBuf>) -> Result<(), CliError> {
    let source = fs::read_to_string(&input)?;
    let generated = compile_component(&source)?;
    let out_path = output.unwrap_or_else(|| input.with_extension("tsx"));
    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(&out_path, generated)?;
    println!("Compiled {} -> {}", input.display(), out_path.display());
    Ok(())
}

fn report_error(err: &CliError) {
    match err {
        CliError::Compile(comp) => match comp {
            CompileError::Component(msg) => eprintln!("error: {}", msg),
            CompileError::Lex(lex) => {
                use ravensone::LexError;
                match lex {
                    LexError::Invalid(span) => {
                        eprintln!(
                            "lexer error at line {}, column {}: invalid token",
                            span.line, span.col
                        );
                    }
                }
            }
            CompileError::Parse(parse) => {
                use ravensone::ParseError;
                match parse {
                    ParseError::Unexpected { span, expected, .. } => {
                        eprintln!(
                            "parse error at line {}, column {}: expected {}",
                            span.line, span.col, expected
                        );
                    }
                    ParseError::UnclosedTag(name, span) => {
                        eprintln!(
                            "parse error at line {}, column {}: unclosed <{}> tag",
                            span.line, span.col, name
                        );
                    }
                    ParseError::MismatchedClose {
                        expected,
                        found,
                        span,
                    } => {
                        eprintln!(
                            "parse error at line {}, column {}: expected </{}> but found </{}>",
                            span.line, span.col, expected, found
                        );
                    }
                }
            }
        },
        CliError::Io(io_err) => eprintln!("io error: {}", io_err),
    }
}
