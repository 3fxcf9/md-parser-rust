#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HrStyle {
    Normal,
    Dotted,
    Dashed,
    Sawtooth,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Header(u8),
    Bold,
    Italic,
    Striked,
    Underline,
    Highlighted,
    LinkStart,
    LinkEnd,
    Text(String),
    ListItem(u8),
    Indent(u8),
    InlineCode(String),
    CodeBlock(String),
    InlineMath(String),
    DisplayMath(String),
    EnvBegin(String),
    EnvEnd,
    NewLine,
    Hr(HrStyle),
    NBSP,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    fn remove_indents(text: String, _level: u8) -> String {
        //TODO: Remove indent in code blocks
        text
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
                // Hr
                '=' if self.next_are(vec!['='; 2]) => {
                    self.advance();
                    self.advance();
                    tokens.push(Token::Hr(HrStyle::Normal));
                }
                '-' if self.next_are(vec!['-'; 2]) => {
                    self.advance();
                    self.advance();
                    tokens.push(Token::Hr(HrStyle::Dashed));
                }
                '.' if self.next_are(vec!['.'; 2]) => {
                    self.advance();
                    self.advance();
                    tokens.push(Token::Hr(HrStyle::Dotted));
                }
                '^' if self.next_are(vec!['^'; 2]) => {
                    self.advance();
                    self.advance();
                    tokens.push(Token::Hr(HrStyle::Sawtooth));
                }

                '\n' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::NewLine);
                    line_begins = true;
                    continue;
                }
                '#' if line_begins => {
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
                '~' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_is('~') {
                        self.advance();
                        tokens.push(Token::Striked);
                    } else {
                        tokens.push(Token::NBSP);
                    }
                }
                '.' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_is('.') {
                        self.advance();
                        tokens.push(Token::Underline);
                    } else {
                        current_text.push(*current);
                    }
                }
                '|' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_is('|') {
                        self.advance();
                        tokens.push(Token::Highlighted);
                    } else {
                        current_text.push(*current);
                    }
                }
                '$' => {
                    push_text(&mut tokens, &mut current_text);
                    tokens.push(Token::InlineMath(self.advance_until('$')));
                }
                '`' => {
                    push_text(&mut tokens, &mut current_text);
                    if self.next_are(vec!['`'; 2]) {
                        self.advance();
                        self.advance();
                        tokens.push(Token::CodeBlock(Lexer::remove_indents(
                            self.advance_until_chars(vec!['`'; 3]),
                            0,
                        )));
                    } else {
                        tokens.push(Token::InlineCode(self.advance_until('`')));
                    }
                }
                '\\' => {
                    if self.next_is('[') {
                        self.advance();
                        push_text(&mut tokens, &mut current_text);
                        tokens.push(Token::DisplayMath(
                            self.advance_until_chars(vec!['\\', ']']),
                        ));
                    } else {
                        current_text.push(*current);
                    }
                }
                '%' => {
                    self.advance_while('%');
                    if self.next_is('\n') {
                        tokens.push(Token::EnvEnd);
                    } else {
                        let name = self.advance_until_one_of_excluded(vec![' ', '\n']);
                        if self.next_is(' ') {
                            self.advance();
                        }
                        tokens.push(Token::EnvBegin(name));
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
    fn next_are(&self, what: Vec<char>) -> bool {
        let mut offset = 0;
        for _ in &what {
            if self.pos + offset < self.input.len()
                && &what[offset] == &self.input.chars().nth(self.pos + offset).unwrap()
            {
                offset += 1
            } else {
                return false;
            }
        }
        return true;
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    fn advance_while(&mut self, c: char) {
        while !self.eof() && self.next_is(c) {
            self.advance();
        }
    }
    fn advance_until(&mut self, until: char) -> String {
        let mut consumed: Vec<char> = vec![];
        while !self.eof() && !self.next_is(until) {
            consumed.push(self.advance().unwrap());
        }
        self.advance();
        consumed.iter().collect::<String>()
    }
    fn advance_until_one_of_excluded(&mut self, until: Vec<char>) -> String {
        let mut consumed: Vec<char> = vec![];
        fn check_next(next: &Option<char>, until: &Vec<char>) -> bool {
            if let Some(n) = next {
                until.contains(n)
            } else {
                false
            }
        }
        while !self.eof() && !check_next(&self.next(), &until) {
            consumed.push(self.advance().unwrap());
        }
        consumed.iter().collect::<String>()
    }
    fn advance_until_chars(&mut self, until: Vec<char>) -> String {
        let mut consumed: Vec<char> = vec![];

        fn check_previous(consumed: &Vec<char>, until: &Vec<char>) -> bool {
            consumed.len() >= until.len() && *until == consumed[(consumed.len() - until.len())..]
        }

        while !self.eof() && !check_previous(&consumed, &until) {
            consumed.push(self.advance().unwrap());
        }

        consumed[..consumed.len() - until.len()]
            .iter()
            .collect::<String>()
    }
}
