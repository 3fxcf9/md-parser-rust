use crate::lexer::Token;

#[derive(Debug)]
pub enum ListType {
    Normal,
}

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
    List {
        list_type: ListType,
        children: Vec<Node>,
    },
    ListItem(Vec<Node>),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    InlineMath(String),
    DisplayMath(String),
    NewLine,
    Paragraph(Vec<Node>),
    Text(String),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self, parsing_list: bool) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        while let Some(current) = self.advance() {
            match current {
                Token::ListItem(indent_level) if !parsing_list => {
                    let mut consumed = vec![Token::ListItem(indent_level)];
                    consumed.extend(self.advance_until(&Token::NewLine));

                    while let Some(tok) = self.tokens.get(0) {
                        match tok {
                            // What’s allowed at line start during list parsing
                            Token::Indent(level) if (level - 2) >= indent_level => (),
                            Token::ListItem(level) if level >= &indent_level => (),
                            Token::NewLine => (),
                            _ => break,
                        }
                        consumed.extend(self.advance_until(&Token::NewLine));
                    }
                    nodes.push(Node::List {
                        list_type: ListType::Normal,
                        children: Parser::new(consumed).parse(true),
                    })
                }
                Token::ListItem(indent_level) if parsing_list => {
                    let mut consumed = self.advance_until(&Token::NewLine);

                    while let Some(tok) = self.tokens.get(0) {
                        match tok {
                            Token::Indent(level) if (level - 2) >= indent_level => (),
                            Token::ListItem(level) if level > &indent_level => (),
                            Token::NewLine => (),
                            _ => break,
                        }
                        consumed.extend(self.advance_until(&Token::NewLine));
                    }

                    nodes.push(Node::ListItem(Parser::new(consumed).parse(false)));
                }

                // Token::ListItem(indent_level) => nodes.push(Node::List{
                //     list_type: ListType::Normal,
                //     children: Parser::new(self.advance_until(&))
                // })
                Token::Header(level) => nodes.push(Node::Header {
                    level,
                    children: Parser::new(self.advance_until(&Token::NewLine)).parse(false),
                }),
                Token::Bold => nodes.push(Node::Bold(
                    Parser::new(self.advance_until(&Token::Bold)).parse(false),
                )),
                Token::Italic => nodes.push(Node::Italic(
                    Parser::new(self.advance_until(&Token::Italic)).parse(false),
                )),
                Token::Text(text) => nodes.push(Node::Text(text.clone())),
                Token::InlineMath(math) => nodes.push(Node::InlineMath(math.to_string())),
                Token::DisplayMath(math) => nodes.push(Node::DisplayMath(math.to_string())),
                Token::NewLine => {
                    if self.next_is(&Token::NewLine) {
                        self.advance();
                        nodes.push(Node::NewLine);
                    } else {
                        nodes.push(Node::Text(" ".to_string()))
                    }
                }
                Token::Indent(_) => (),
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
        if let Some(next) = self.advance() {
            consumed.push(next);
        }
        consumed
    }
}
