#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Header(u8),
    Bold,
    Italic,
    LinkStart,
    LinkEnd,
    Text(String),
    ListItem(u8),
    Code,
    InlineMath(String),
    DisplayMath(String),
    NewLine,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
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

        while let Some(current) = &self.advance() {
            match current {
                '\n' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::NewLine);
                }
                '#' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::Header(self.advance_until(' ').len() as u8));
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
                        tokens.push(Token::DisplayMath(self.advance_until_two(('\\', ']'))));
                    } else {
                        tokens.push(Token::Text(current.to_string()));
                    }
                }
                _ => current_text.push(*current),
            }
        }

        push_text(&mut tokens, &mut current_text);

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
        consumed.push(self.advance().unwrap());
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
