//! TSX codegen

use crate::ast::{Attr, AttrValue, Document, Node};

pub fn to_tsx(doc: &Document) -> String {
    let mut out = String::new();
    for node in &doc.children {
        render_node(node, &mut out);
    }
    out
}

fn render_node(node: &Node, out: &mut String) {
    match node {
        Node::Element {
            name,
            attrs,
            children,
        } => {
            out.push('<');
            out.push_str(name);
            render_attrs(attrs, out);
            if children.is_empty() {
                out.push_str(" />");
            } else {
                out.push('>');
                for child in children {
                    render_node(child, out);
                }
                out.push_str("</");
                out.push_str(name);
                out.push('>');
            }
        }
        Node::Text(text) => out.push_str(text),
        Node::Expr(code) => {
            out.push('{');
            out.push_str(code);
            out.push('}');
        }
    }
}

fn render_attrs(attrs: &[Attr], out: &mut String) {
    for attr in attrs {
        out.push(' ');
        out.push_str(&attr.name);
        out.push('=');
        match &attr.value {
            AttrValue::Str(value) => {
                out.push('"');
                out.push_str(&escape_string(value));
                out.push('"');
            }
            AttrValue::Expr(code) => {
                out.push('{');
                out.push_str(code);
                out.push('}');
            }
        }
    }
}

fn escape_string(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            other => other.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Document, Node};

    #[test]
    fn renders_simple_tree() {
        let doc = Document {
            children: vec![Node::Element {
                name: "div".into(),
                attrs: vec![],
                children: vec![Node::Text("Hello".into())],
            }],
        };
        assert_eq!(to_tsx(&doc), "<div>Hello</div>");
    }

    #[test]
    fn renders_empty_element_self_closing() {
        let doc = Document {
            children: vec![Node::Element {
                name: "img".into(),
                attrs: vec![],
                children: vec![], // no children -> self-closing
            }],
        };
        assert_eq!(to_tsx(&doc), "<img />");
    }
}

// Optional placeholder at module scope (NOT inside tests)
#[allow(dead_code)]
pub fn init() -> &'static str {
    "initialized"
}
