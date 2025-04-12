use crate::lexer::{HrStyle, Token};

#[derive(Debug)]
pub enum ListType {
    Normal,
}

#[derive(Debug)]
pub enum EnvType {
    Definition,
    Theorem,
    Corollary,
    Lemma,
    Remark,
    Example,
    Exercise,
    Fold,
    Conceal,
}

#[derive(Debug)]
pub enum Node {
    Header {
        level: u8,
        children: Vec<Node>,
    },
    Bold(Vec<Node>),
    Italic(Vec<Node>),
    Striked(Vec<Node>),
    Underline(Vec<Node>),
    Highlighted(Vec<Node>),
    Link {
        url: String,
        childen: Vec<Node>,
    },
    List {
        list_type: ListType,
        children: Vec<Node>,
    },
    ListItem(Vec<Node>),
    InlineCode(String),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    InlineMath(String),
    DisplayMath(String),
    Env {
        environment_type: EnvType,
        environment_arg: Option<Vec<Node>>,
        children: Vec<Node>,
    },
    NewLine,
    Paragraph(Vec<Node>),
    Text(String),
    Hr(HrStyle),
    NBSP,
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn preprocess(tokens: Vec<Token>) -> Vec<Token> {
        match tokens.get(0) {
            Some(Token::Header(_)) => tokens,
            _ => [vec![Token::NewLine; 2], tokens].concat(),
        }
    }

    pub fn parse(&mut self, parsing_list: bool) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        'parse: while let Some(current) = self.advance() {
            match current {
                Token::ListItem(indent_level) if !parsing_list => {
                    let mut consumed = vec![Token::ListItem(indent_level)];
                    consumed.extend(self.advance_until_included(&Token::NewLine));

                    while let Some(tok) = self.tokens.get(0) {
                        match tok {
                            // What’s allowed at line start during list parsing
                            Token::Indent(level) if (level - 2) >= indent_level => (),
                            Token::ListItem(level) if level >= &indent_level => (),
                            Token::NewLine => (),
                            _ => break,
                        }
                        consumed.extend(self.advance_until_included(&Token::NewLine));
                    }

                    self.tokens.insert(0, Token::NewLine);
                    self.tokens.insert(0, Token::NewLine);

                    nodes.push(Node::List {
                        list_type: ListType::Normal,
                        children: Parser::new(consumed).parse(true),
                    })
                }
                Token::ListItem(indent_level) if parsing_list => {
                    let should_include_paragraph = if let Some(Token::Text(_)) = self.tokens.get(0)
                    {
                        true
                    } else {
                        false
                    };
                    let mut consumed = self.advance_until_included(&Token::NewLine);

                    while let Some(tok) = self.tokens.get(0) {
                        match tok {
                            Token::Indent(level) if (level - 2) >= indent_level => (),
                            Token::ListItem(level) if level > &indent_level => (),
                            Token::NewLine => (),
                            _ => break,
                        }
                        consumed.extend(self.advance_until_included(&Token::NewLine));
                    }

                    if should_include_paragraph {
                        consumed.insert(0, Token::NewLine);
                        consumed.insert(0, Token::NewLine);
                    }

                    nodes.push(Node::ListItem(Parser::new(consumed).parse(false)));
                }

                // Token::ListItem(indent_level) => nodes.push(Node::List{
                //     list_type: ListType::Normal,
                //     children: Parser::new(self.advance_until(&))
                // })
                Token::Header(level) => nodes.push(Node::Header {
                    level,
                    children: Parser::new(self.advance_until_and_stop_before(&Token::NewLine))
                        .parse(false),
                }),
                Token::Bold => nodes.push(Node::Bold(
                    Parser::new(self.advance_until(&Token::Bold)).parse(false),
                )),
                Token::Italic => nodes.push(Node::Italic(
                    Parser::new(self.advance_until(&Token::Italic)).parse(false),
                )),
                Token::Striked => nodes.push(Node::Striked(
                    Parser::new(self.advance_until(&Token::Striked)).parse(false),
                )),
                Token::Underline => nodes.push(Node::Underline(
                    Parser::new(self.advance_until(&Token::Underline)).parse(false),
                )),
                Token::Highlighted => nodes.push(Node::Highlighted(
                    Parser::new(self.advance_until(&Token::Highlighted)).parse(false),
                )),
                Token::Text(text) => nodes.push(Node::Text(text.clone())),
                Token::InlineMath(math) => nodes.push(Node::InlineMath(math.to_string())),
                Token::DisplayMath(math) => nodes.push(Node::DisplayMath(math.to_string())),
                Token::InlineCode(code) => nodes.push(Node::InlineCode(code.to_string())),
                Token::CodeBlock(code) => nodes.push(Node::CodeBlock {
                    language: None,
                    code: code.to_string(),
                }),
                Token::EnvBegin(name) => {
                    let env_type = match name.as_str() {
                        "def" => EnvType::Definition,
                        "thm" => EnvType::Theorem,
                        "cor" => EnvType::Corollary,
                        "lemma" => EnvType::Lemma,
                        "rem" => EnvType::Remark,
                        "eg" => EnvType::Example,
                        "ex" => EnvType::Exercise,
                        "fold" => EnvType::Fold,
                        "conceal" => EnvType::Conceal,
                        _ => continue 'parse,
                    };

                    let line = self.advance_until(&Token::NewLine);
                    let arg = if line.len() > 0 {
                        Some(Parser::new(line).parse(false))
                    } else {
                        None
                    };

                    let mut consumed: Vec<Token> = vec![];
                    let mut count: i8 = 0;
                    while !self.eof() && (!self.next_is(&Token::EnvEnd) || count != 0) {
                        let current = self.tokens.remove(0);
                        match current {
                            Token::EnvBegin(_) => count += 1,
                            Token::EnvEnd => count -= 1,
                            _ => (),
                        }
                        consumed.push(current);
                    }
                    self.advance();

                    nodes.push(Node::Env {
                        environment_type: env_type,
                        environment_arg: arg,
                        children: Parser::new(consumed).parse(false),
                    })
                }
                Token::NewLine => {
                    if self.next_is(&Token::NewLine) {
                        self.advance();
                        let mut consumed: Vec<Token> = vec![];

                        while !self.eof()
                            && !(self.next_is(&Token::NewLine)
                                && self.next_n_is(&Token::NewLine, 1))
                        {
                            // Stop at one of [list, header, code block, hr, env] and cancel paragraph creation
                            match self.tokens.get(0).unwrap() {
                                Token::EnvBegin(_)
                                | Token::EnvEnd
                                | Token::ListItem(_)
                                | Token::Header(_)
                                | Token::CodeBlock(_)
                                | Token::DisplayMath(_)
                                | Token::Hr(_) => continue 'parse,
                                _ => consumed.push(self.advance().unwrap()),
                            }
                        }

                        if !consumed.is_empty() {
                            nodes.push(Node::Paragraph(Parser::new(consumed).parse(false)));
                        }
                    } else {
                        nodes.push(Node::NewLine)
                    }
                }
                Token::NBSP => nodes.push(Node::NBSP),
                Token::Hr(style) => nodes.push(Node::Hr(style)),
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
    fn next_n_is(&self, what: &Token, offset: usize) -> bool {
        self.tokens.get(offset) == Some(what)
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
    fn advance_until_included(&mut self, until: &Token) -> Vec<Token> {
        let mut consumed: Vec<Token> = vec![];
        while !self.eof() && !self.next_is(until) {
            consumed.push(self.tokens.remove(0));
        }

        if let Some(next) = self.advance() {
            consumed.push(next);
        }
        consumed
    }
    fn advance_until_and_stop_before(&mut self, until: &Token) -> Vec<Token> {
        let mut consumed: Vec<Token> = vec![];
        while !self.eof() && !self.next_is(until) {
            consumed.push(self.tokens.remove(0));
        }

        consumed
    }
    fn advance_until_tokens(&mut self, until: &Vec<Token>) -> Vec<Token> {
        let mut consumed: Vec<Token> = vec![];

        fn check_previous(consumed: &Vec<Token>, until: &Vec<Token>) -> bool {
            consumed.len() >= until.len() && *until == consumed[(consumed.len() - until.len())..]
        }

        while !self.eof() && !check_previous(&consumed, &until) {
            consumed.push(self.advance().unwrap());
        }

        if consumed.len() >= until.len() {
            for u in until {
                self.tokens.insert(0, u.clone());
            }
            consumed[..consumed.len() - until.len()].to_vec()
        } else {
            vec![]
        }
    }
}
