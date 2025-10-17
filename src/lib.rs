//! RavensOne compiler library.

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod token;

pub use codegen::to_tsx;
pub use lexer::lex;
pub use parser::parse;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("component syntax error: {0}")]
    Component(String),
    #[error("lex error: {0}")]
    Lex(#[from] lexer::LexError),
    #[error("parse error: {0}")]
    Parse(#[from] parser::ParseError),
}

pub fn compile_component(src: &str) -> Result<String, CompileError> {
    let component = parse_component(src)?;
    let markup = extract_markup(&component.body)?;
    let tokens = lex(&markup)?;
    let doc = parse(&tokens, &markup)?;
    let tsx = to_tsx(&doc);
    Ok(render_component(&component.name, &component.params, &tsx))
}

struct Component {
    name: String,
    params: Vec<String>,
    body: String,
}

fn parse_component(src: &str) -> Result<Component, CompileError> {
    let trimmed = src.trim();
    let rest = trimmed
        .strip_prefix("component")
        .ok_or_else(|| CompileError::Component("expected `component` keyword".into()))?
        .trim_start();

    let paren_start = rest
        .find('(')
        .ok_or_else(|| CompileError::Component("missing parameter list".into()))?;
    let name = rest[..paren_start].trim();
    if name.is_empty() {
        return Err(CompileError::Component("missing component name".into()));
    }

    let (params_raw, after_params) = extract_parens(&rest[paren_start..])?;
    let params = parse_params(params_raw);

    let (body, _) = extract_block(after_params.trim_start())?;

    Ok(Component {
        name: name.to_string(),
        params,
        body: body.to_string(),
    })
}

fn extract_markup(body: &str) -> Result<String, CompileError> {
    let return_idx = body
        .find("return")
        .ok_or_else(|| CompileError::Component("missing `return` in component body".into()))?;
    let after_return = &body[return_idx + "return".len()..];
    let mut markup = after_return.trim();
    if markup.ends_with(';') {
        markup = markup[..markup.len() - 1].trim_end();
    }
    if markup.starts_with('(') && markup.ends_with(')') {
        markup = markup[1..markup.len() - 1].trim();
    }
    if markup.is_empty() {
        return Err(CompileError::Component("empty return body".into()));
    }
    Ok(markup.to_string())
}

fn render_component(name: &str, params: &[String], tsx: &str) -> String {
    let params_binding = if params.is_empty() {
        "()".to_string()
    } else {
        format!("({{ {} }})", params.join(", "))
    };

    let mut output = String::new();
    output.push_str(&format!("export function {}{} {{\n", name, params_binding));
    output.push_str("  return ");
    output.push_str(tsx);
    output.push_str(";\n}\n");
    output
}

fn extract_parens(input: &str) -> Result<(&str, &str), CompileError> {
    if !input.starts_with('(') {
        return Err(CompileError::Component("expected `(`".into()));
    }
    let mut depth = 0usize;
    for (idx, ch) in input.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let inside = &input[1..idx];
                    let rest = &input[idx + 1..];
                    return Ok((inside, rest));
                }
            }
            _ => {}
        }
    }
    Err(CompileError::Component(
        "unclosed `(` in parameter list".into(),
    ))
}

fn extract_block(input: &str) -> Result<(&str, &str), CompileError> {
    if !input.starts_with('{') {
        return Err(CompileError::Component("expected `{`".into()));
    }
    let mut depth = 0usize;
    for (idx, ch) in input.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let inside = &input[1..idx];
                    let rest = &input[idx + 1..];
                    return Ok((inside, rest));
                }
            }
            _ => {}
        }
    }
    Err(CompileError::Component(
        "unclosed `{` in component body".into(),
    ))
}

fn parse_params(params: &str) -> Vec<String> {
    params
        .split(',')
        .filter_map(|param| {
            let name = param.split(':').next()?.trim();
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect()
}
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_simple_component() {
        let source = "component App() { return <div>Hello</div> }";
        let output = compile_component(source).unwrap();
        assert!(output.contains("export function App"));
        assert!(output.contains("<div>Hello</div>"));
    }
}
