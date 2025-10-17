use ravensone::{lex, parse, to_tsx};

mod common;

#[test]
fn compiles_basic_program() {
    run_fixture("basic/input.raven", "basic/expected.tsx");
}

#[test]
fn compiles_button_component() {
    run_fixture("components/button.raven", "components/button.expected.tsx");
}

fn run_fixture(input: &str, expected: &str) {
    let source = common::read_fixture(input);
    let expected_tsx = common::read_fixture(expected);
    let component = parse_component(&source);
    let tokens = lex(&component.markup).expect("lexing succeeded");
    let doc = parse(&tokens, &component.markup).expect("parsing succeeded");
    let tsx_body = to_tsx(&doc);
    let output = render_component(&component.name, &component.params, &tsx_body);
    assert_eq!(output, expected_tsx);
}

struct Component {
    name: String,
    params: Vec<String>,
    markup: String,
}

fn parse_component(src: &str) -> Component {
    let trimmed = src.trim();
    let rest = trimmed.strip_prefix("component").expect("component keyword").trim_start();
    let paren_idx = rest.find('(').expect("parameter list");
    let name = rest[..paren_idx].trim().to_string();
    let (params_src, after_params) = extract_parens(&rest[paren_idx..]);
    let params = params_src
        .split(',')
        .filter_map(|param| {
            let name = param.split(':').next()?.trim();
            if name.is_empty() { None } else { Some(name.to_string()) }
        })
        .collect();
    let body = extract_block(after_params.trim_start());
    let markup = extract_markup(&body);
    Component { name, params, markup }
}

fn extract_parens(input: &str) -> (&str, &str) {
    assert!(input.starts_with('('));
    let mut depth = 0;
    for (idx, ch) in input.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return (&input[1..idx], &input[idx + 1..]);
                }
            }
            _ => {}
        }
    }
    panic!("unclosed parens");
}

fn extract_block(input: &str) -> String {
    assert!(input.starts_with('{'));
    let mut depth = 0;
    for (idx, ch) in input.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return input[1..idx].to_string();
                }
            }
            _ => {}
        }
    }
    panic!("unclosed block");
}

fn extract_markup(body: &str) -> String {
    let return_idx = body.find("return").expect("return keyword");
    let mut markup = body[return_idx + "return".len()..].trim();
    if markup.ends_with(';') {
        markup = markup[..markup.len() - 1].trim_end();
    }
    if markup.starts_with('(') && markup.ends_with(')') {
        markup = markup[1..markup.len() - 1].trim();
    }
    markup.to_string()
}

fn render_component(name: &str, params: &[String], body: &str) -> String {
    let params_binding = if params.is_empty() {
        "()".to_string()
    } else {
        format!("({{ {} }})", params.join(", "))
    };
    format!("export function {}{} {{\n  return {};\n}}\n", name, params_binding, body)
    let fixture = common::fixture("basic/input.raven");
    assert!(fixture.contains("basic"));
}
