#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue {
    Str(String),
    Expr(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attr {
    pub name: String,
    pub value: AttrValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element {
        name: String,
        attrs: Vec<Attr>,
        children: Vec<Node>,
    },
    Text(String),
    Expr(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub children: Vec<Node>,
}
