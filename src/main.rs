mod lexer;
mod parser;
use lexer::{Lexer, Token};
use parser::{Node, Parser};

fn nodes_to_html(nodes: Vec<Node>) -> String {
    let mut node_str: Vec<String> = vec![];
    for node in nodes {
        node_str.push(match node {
            Node::NewLine => "<br/>".to_string(),
            Node::Header { level, children } => {
                format!("<h{level}>{}</h{level}>", nodes_to_html(children))
            }
            Node::Text(text) => text.to_string(),
            Node::Bold(children) => format!("<strong>{}</strong>", nodes_to_html(children)),
            Node::Italic(children) => format!("<em>{}</em>", nodes_to_html(children)),
            Node::DisplayMath(math) => format!("<display-math>{}</display-math>", math),
            Node::InlineMath(math) => format!("<inline-math>{}</inline-math>", math),
            Node::List {
                list_type: _,
                children,
            } => format!("<ul>{}</ul>", nodes_to_html(children)),
            Node::ListItem(children) => format!("<li>{}</li>", nodes_to_html(children)),
            _ => todo!(),
        });
    }
    node_str.join("")
}

fn main() {
    // let test = "# Header $1$\n\n### Level _three_\n_this_ is *italic* and **this** is __bold__ text.\n\\[\n\\sqrt{test}\n\\]\ntext";
    // let test = "text and _italic with **bold**nested_.";
    // let test = "# Header one\n\n### Level _three_\n_this_ is *italic* and **this** is __bold__ text.";
    // let test = "hello\n- first\n  second\n    - nested\n      test\n- third\ntext";

    let test = include_str!("test.md");

    let mut tokenizer = Lexer::new(test);
    let tokens = tokenizer.tokenize();

    for token in &tokens {
        println!("{}", token);
    }

    let mut parser = Parser::new(tokens);
    let nodes = parser.parse(false);
    dbg!(&nodes);

    println!("IN: {:?}", test);
    println!("OUT: {:?}", nodes_to_html(nodes));
}
