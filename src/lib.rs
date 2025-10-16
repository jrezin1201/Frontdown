//! RavensOne compiler library.

pub mod analyzer;
pub mod ast;
pub mod bundler;
pub mod codegen;
pub mod codegen_enhanced;
pub mod config;
pub mod devserver;
pub mod env;
pub mod errors;
pub mod formatter;
pub mod jsx;
pub mod lexer;
pub mod linter;
pub mod middleware;
pub mod module;
pub mod optimizer;
pub mod parser;
pub mod project;
pub mod route_parser;
pub mod schema;
pub mod stdlib;
pub mod token;
pub mod typechecker;

/// Returns the current RavensOne library version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
