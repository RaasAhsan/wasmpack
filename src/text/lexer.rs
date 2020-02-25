use regex::Regex;

// The lexer consumes a stream of Unicode characters and produces a stream of tokens that represent lexemes. Whitespace tokens are ignored on their own.
// Unicode characters may only appear in string literals and comments; the grammar is otherwise formed from UTF-8 characters.
// https://webassembly.github.io/spec/core/text/lexical.html#
// The lexical format of tokens are specified by regular languages which admit the following production rules: terminals and nonterminals
// A -> a, A -> aB

// The next token is taken to be the longest possible sequence of characters defined by the grammar.

// How to express alternatives of a production more effectively?
// Structure: One function for each nonterminal. One function per alternative for a production. Can be collapsed into one if it makes sense.

type Lex<A> = Option<(A, usize)>;

struct State {
    input: String,
    cursor: usize
}

impl State {

    fn advance(&mut self, offset: usize) {
        self.cursor += offset;
    }

    fn rest(&self) -> String {
        self.input.chars().skip(self.cursor).collect()
    }

    fn eof(&self) -> bool {
        self.cursor == self.input.len()
    }

}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    // TODO: Are 32-bit types sufficient here?
    Unsigned(u32),
    Signed(i32),
    Float(f32),
    String(String),
    Id(String),
    LeftParen,
    RightParen,
    Reserved(String)
}

#[derive(Debug)]
pub struct Keyword {

}

pub fn lex(input: String) -> Result<Vec<Token>, String> {
    let mut state = State {
        input: input.clone(),
        cursor: 0
    };

    let mut tokens: Vec<Token> = vec![];
    let mut no_match = false;
    while !state.eof() && !no_match {
        match lex_token(&state) {
            None => {},
            Some(t) => {
                tokens.push(t.0);
                state.advance(t.1);
                continue;
            }
        }

        match lex_space(&state) {
            None => {},
            Some(t) => {
                state.advance(t.1);
                continue;
            }
        }

        no_match = true;
    }

    if no_match {
        Err("Token not matched".to_string())
    } else {
        Ok(tokens)
    }
}

// TODO: include comments
fn lex_space(state: &State) -> Lex<()> {
    let re = Regex::new(r"^[\n\t\r ]+").unwrap();

    match re.find(state.rest().as_ref()) {
        None => None,
        Some(mat) => {
            let str = mat.as_str();
            Some(((), str.len()))
        }
    }
}

fn choose<A>(current: Lex<A>, next: Lex<A>) -> Lex<A> {
    match current {
        None => next,
        Some(c) => match next {
            None => Some(c),
            Some(n) => if c.1 >= n.1 {
                Some(c)
            } else {
                Some(n)
            }
        }
    }
}

fn lex_token(state: &State) -> Lex<Token> {
    let mut token: Lex<Token> = None;
    token = choose(token, lex_unsigned(state));
    token = choose(token, lex_left_paren(state));
    token = choose(token, lex_right_paren(state));
    token
}

fn lex_left_paren(state: &State) -> Lex<Token> {
    if state.rest().starts_with("(") {
        Some((Token::LeftParen, 1))
    } else {
        None
    }
}

fn lex_right_paren(state: &State) -> Lex<Token> {
    if state.rest().starts_with(")") {
        Some((Token::RightParen, 1))
    } else {
        None
    }
}

fn lex_keyword(input: &str) -> u32 {
    234
}

fn lex_reserved(input: &str) -> u32 {
    34
}

fn lex_id(state: &State) -> Lex<String> {
    let re = Regex::new(r"^\$[0-9A-Za-z!#$%&'*+\-./:<=>?@\\^_`|~]+").unwrap();
    match re.find(state.rest().as_ref()) {
        None => None,
        Some(mat) => {
            let str = mat.as_str();
            Some((str.to_string(), str.len()))
        }
    }
}

// TODO: Use lazy_static to avoid compiling the regex on every invocation
// TODO: The regex should admit underscores
fn lex_unsigned(state: &State) -> Lex<Token> {
    let mut u: Lex<u32> = None;
    u = choose(u, lex_unsigned_dec(state));
    u = choose(u, lex_unsigned_hex(state));
    u.map(|n| (Token::Unsigned(n.0), n.1))
}

fn lex_unsigned_dec(state: &State) -> Lex<u32> {
    let re = Regex::new(r"^[0-9]+").unwrap();
    match re.find(state.rest().as_ref()) {
        None => None,
        Some(mat) => {
            let str = mat.as_str();
            match u32::from_str_radix(str, 10) {
                Err(e) => None,
                Ok(z) => Some((z, str.len()))
            }
        }
    }
}

fn lex_unsigned_hex(state: &State) -> Lex<u32> {
    let re2 = Regex::new(r"^0x[0-9A-Fa-f]+").unwrap();
    match re2.find(state.rest().as_ref()) {
        None => None,
        Some(mat) => {
            let str = mat.as_str();
            match u32::from_str_radix(&str[2..], 16) {
                Err(e) => None,
                Ok(z) => Some((z, str.len()))
            }
        }
    }
}
