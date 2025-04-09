mod lexer;
mod parser;
use lexer::{Lexer, Token};
use parser::{Node, Parser};
use std::fs;

fn nodes_to_html(nodes: &Vec<Node>) -> String {
    let mut node_str: Vec<String> = vec![];
    for node in nodes {
        node_str.push(match node {
            Node::NewLine => "<br/>".to_string(),
            Node::Header { level, children } => {
                format!("<h{level}>{}</h{level}>", nodes_to_html(children))
            }
            Node::Text(text) => text.to_string(),
            Node::Paragraph(children) => format!("<p>{}</p>", nodes_to_html(children)),
            Node::Bold(children) => format!("<strong>{}</strong>", nodes_to_html(children)),
            Node::Italic(children) => format!("<em>{}</em>", nodes_to_html(children)),
            Node::Striked(children) => format!("<s>{}</s>", nodes_to_html(children)),
            Node::Underline(children) => format!("<u>{}</u>", nodes_to_html(children)),
            Node::Highlighted(children) => format!("<mark>{}</mark>", nodes_to_html(children)),
            Node::InlineMath(math) => format!("<span class=\"math-inline\">{}</span>", math),
            Node::DisplayMath(math) => format!("<span class=\"math-display\">{}</span>", math),
            Node::InlineCode(code) => format!("<code class=\"inline\">{}</code>", code),
            Node::CodeBlock { language, code } => {
                format!(
                    "<pre><code class=\"block\"{}>{code}</code></pre>",
                    if let Some(lang) = language {
                        format!(" lang=\"{}\"", lang)
                    } else {
                        String::new()
                    }
                )
            }
            Node::List {
                list_type: _,
                children,
            } => format!("<ul>{}</ul>", nodes_to_html(children)),
            Node::ListItem(children) => format!("<li>{}</li>", nodes_to_html(children)),
            Node::NBSP => "&nbsp;".to_string(),
            _ => todo!(),
        });
    }
    node_str.join("")
}

fn main() {
    let input = include_str!("test.md");

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    dbg!(&tokens);

    let mut parser = Parser::new(Parser::preprocess(tokens));
    let nodes = parser.parse(false);

    dbg!(&nodes);

    let out = nodes_to_html(&nodes);

    println!("IN: {:?}", input);
    println!("OUT: {:?}", out);

    let head = "<style>p {padding: 1rem; border: 1px dashed red;} .math-inline{font-family: monospace; font-weight: bold; color: grey;} .math-display{font-family: monospace; font-weight: bold; color: grey; display: block; padding: 1rem; text-align: center; font-size: 2rem;}</style>";

    fs::write("out.html", format!("<head>{head}</head><body>{out}</body>"))
        .expect("Could not write to file");
}
