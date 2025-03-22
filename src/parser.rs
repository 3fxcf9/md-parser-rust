use crate::lexer::Token;

#[derive(Debug)]
pub enum Node {
    Header {
        level: u8,
        children: Vec<Node>,
    },
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Link {
        url: String,
        childen: Vec<Node>,
    },
    ListItem(Vec<Node>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    InlineMath(String),
    DisplayMath(String),
    NewLine,
    Text(String),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        while let Some(current) = self.advance() {
            match current {
                Token::NewLine => nodes.push(Node::NewLine),
                Token::Header(level) => nodes.push(Node::Header {
                    level,
                    children: Parser::new(self.advance_until(&Token::NewLine)).parse(),
                }),
                Token::Bold => nodes.push(Node::Bold(
                    Parser::new(self.advance_until(&Token::Bold)).parse(),
                )),
                Token::Italic => nodes.push(Node::Italic(
                    Parser::new(self.advance_until(&Token::Italic)).parse(),
                )),
                Token::Text(text) => nodes.push(Node::Text(text.clone())),
                Token::InlineMath(math) => nodes.push(Node::InlineMath(math.to_string())),
                Token::DisplayMath(math) => nodes.push(Node::DisplayMath(math.to_string())),
                _ => todo!(),
            }
        }

        nodes
    }

    fn advance(&mut self) -> Option<Token> {
        if self.eof() {
            None
        } else {
            Some(self.tokens.remove(0))
        }
    }
    fn next_is(&self, what: &Token) -> bool {
        self.tokens.get(0) == Some(what)
    }
    fn eof(&self) -> bool {
        self.tokens.is_empty()
    }
    fn advance_until(&mut self, until: &Token) -> Vec<Token> {
        let mut consumed: Vec<Token> = vec![];
        while !self.eof() && !self.next_is(until) {
            consumed.push(self.tokens.remove(0));
        }
        self.advance();
        consumed
    }
}
