#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Header(u8),
    Bold,
    Italic,
    LinkStart,
    LinkEnd,
    Text(String),
    ListItem(u8),
    Indent(u8),
    Code,
    InlineMath(String),
    DisplayMath(String),
    NewLine,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Header(level) => {
                write!(f, "\x1b[36m[H{}] \x1b[0m", level)
            }
            Token::Bold => {
                write!(f, "\x1b[1m[BOLD] \x1b[0m")
            }
            Token::Italic => {
                write!(f, "\x1b[3m[ITALIC] \x1b[0m")
            }
            Token::LinkStart => {
                write!(f, "\x1b[35m[LINK_START] \x1b[0m")
            }
            Token::LinkEnd => {
                write!(f, "\x1b[35m[LINK_END] \x1b[0m")
            }
            Token::Text(content) => {
                write!(f, "\x1b[37m[TEXT: {}] \x1b[0m", content)
            }
            Token::ListItem(level) => {
                write!(f, "\x1b[34m[LI{}] \x1b[0m", level)
            }
            Token::Indent(level) => {
                write!(f, "\x1b[90m[INDENT{}] \x1b[0m", level)
            }
            Token::Code => {
                write!(f, "\x1b[32m[CODE] \x1b[0m")
            }
            Token::InlineMath(content) => {
                write!(f, "\x1b[33m[MATH: {}] \x1b[0m", content)
            }
            Token::DisplayMath(content) => {
                write!(f, "\x1b[33;1m[MATH_DISP: {}] \x1b[0m", content)
            }
            Token::NewLine => {
                write!(f, "\x1b[90m[NL] \x1b[0m")
            }
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut current_text: Vec<char> = vec![];
        fn push_text(tokens: &mut Vec<Token>, current_text: &mut Vec<char>) {
            if !current_text.is_empty() {
                tokens.push(Token::Text(current_text.iter().collect::<String>()));
                current_text.clear();
            }
        }

        let mut line_begins = true;

        while let Some(current) = &self.advance() {
            match current {
                ' ' => {
                    if line_begins {
                        let mut indent_level: u8 = 1;
                        while !self.eof() && self.next_is(' ') {
                            indent_level += 1;
                            self.advance();
                        }
                        if self.next_is('-') {
                            tokens.push(Token::ListItem(indent_level));
                            self.advance(); // '-'
                            self.advance(); // ' '
                        } else {
                            tokens.push(Token::Indent(indent_level));
                        }
                    } else {
                        current_text.push(*current);
                    }
                }
                '-' if line_begins && self.next_is(' ') => {
                    tokens.push(Token::ListItem(0));
                    self.advance();
                }
                '\n' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::NewLine);
                    line_begins = true;
                    continue;
                }
                '#' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::Header(self.advance_until(' ').len() as u8 + 1));
                }
                '*' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_is('*') {
                        self.advance();
                        tokens.push(Token::Bold);
                    } else {
                        tokens.push(Token::Italic);
                    }
                }
                '_' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_is('_') {
                        self.advance();
                        tokens.push(Token::Bold);
                    } else {
                        tokens.push(Token::Italic);
                    }
                }
                '$' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::InlineMath(self.advance_until('$')));
                }
                '\\' => {
                    if self.next_is('[') {
                        self.advance();
                        push_text(&mut tokens, &mut current_text);
                        tokens.push(Token::DisplayMath(self.advance_until_two(('\\', ']'))));
                    } else {
                        current_text.push(*current);
                    }
                }
                _ => {
                    current_text.push(*current);
                }
            }
            line_begins = false;
        }

        if !current_text.is_empty() {
            tokens.push(Token::Text(current_text.iter().collect::<String>()));
            current_text.clear();
        }

        tokens
    }

    fn advance(&mut self) -> Option<char> {
        self.pos += 1;
        self.input.chars().nth(self.pos - 1)
    }
    fn next(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }
    fn next_is(&self, what: char) -> bool {
        if let Some(next_char) = self.input.chars().nth(self.pos) {
            return next_char == what;
        }
        return false;
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    fn advance_until(&mut self, until: char) -> String {
        let mut consumed: Vec<char> = vec![];
        while !self.eof() && !self.next_is(until) {
            consumed.push(self.advance().unwrap());
        }
        self.advance().unwrap();
        consumed.iter().collect::<String>()
    }
    fn advance_until_two(&mut self, until: (char, char)) -> String {
        let mut consumed: Vec<char> = vec![];
        let mut previous_was: Option<char> = None;
        while (!self.eof()) && !(previous_was == Some(until.0) && self.next_is(until.1)) {
            let current = self.advance().unwrap();
            previous_was = Some(current);
            consumed.push(current);
        }
        self.advance();

        consumed[..consumed.len() - 1].iter().collect::<String>()
    }
}
