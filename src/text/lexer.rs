use regex::Regex;

// The lexer consumes a stream of Unicode characters and produces a stream of tokens that represent lexemes. Whitespace tokens are ignored on their own.
// Unicode characters may only appear in string literals and comments; the grammar is otherwise formed from UTF-8 characters.
// https://webassembly.github.io/spec/core/text/lexical.html#
// The lexical format of tokens are specified by regular languages which admit the following production rules: terminals and nonterminals
// A -> a, A -> aB

// The next token is taken to be the longest possible sequence of characters defined by the grammar.

// How to express alternatives of a production more effectively?
// Structure: One function for each nonterminal. One function per alternative for a production. Can be collapsed into one if it makes sense.

type Lex<A> = Result<A, String>;

struct State {
    input: String,
    cursor: usize,
    tokens: Vec<Token>
}

impl State {

    fn advance(&mut self, offset: usize) {
        self.cursor += offset;
    }

    fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
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
        cursor: 0,
        tokens: vec![]
    };

    let mut no_match = false;
    while !state.eof() && !no_match {
        if lex_token(&mut state) {
            continue;
        }

        if lex_space(&mut state) {
            continue;
        }

        no_match = true;
    }

    if no_match {
        Err("Token not matched".to_string())
    } else {
        Ok(state.tokens)
    }
}

// TODO: include comments
fn lex_space(state: &mut State) -> bool {
    let re = Regex::new(r"^[\n\t\r ]+").unwrap();

    match re.find(state.rest().as_ref()) {
        None => false,
        Some(mat) => {
            let str = mat.as_str();
            state.advance(str.len());
            true
        }
    }
}

fn lex_token(state: &mut State) -> bool {
    if lex_unsigned(state) {
        return true;
    }

    if lex_left_paren(state) {
        return true;
    }

    if lex_right_paren(state) {
        return true;
    }

    return false;
}

fn lex_left_paren(state: &mut State) -> bool {
    if state.rest().starts_with("(") {
        state.advance(1);
        state.push_token(Token::LeftParen);
        true
    } else {
        false
    }
}

fn lex_right_paren(state: &mut State) -> bool {
    if state.rest().starts_with(")") {
        state.advance(1);
        state.push_token(Token::LeftParen);
        true
    } else {
        false
    }
}

fn lex_keyword(input: &str) -> u32 {
    234
}

fn lex_reserved(input: &str) -> u32 {
    34
}

fn lex_id(state: &mut State) -> bool {
    let re = Regex::new(r"^\$[0-9A-Za-z!#$%&'*+\-./:<=>?@\\^_`|~]+").unwrap();

    match re.find(state.rest().as_ref()) {
        None => false,
        Some(mat) => {
            let str = mat.as_str();
            state.advance(str.len());
            state.push_token(Token::Id(str.to_string()));
            true
        }
    }
}

// TODO: Use lazy_static to avoid compiling the regex on every invocation
// TODO: The regex should admit underscores
// TODO: Take the longer of the two
// input case (((((12423)))0xff))))))
fn lex_unsigned(state: &mut State) -> bool {
    let re = Regex::new(r"^[0-9]+").unwrap();
    match re.find(state.rest().as_ref()) {
        None => {},
        Some(mat) => {
            let str = mat.as_str();
            match u32::from_str_radix(str, 10) {
                Err(e) => {},
                Ok(z) => {
                    state.advance(str.len());
                    state.push_token(Token::Unsigned(z));
                    return true;
                }
            }
        }
    }

    let re2 = Regex::new(r"^0x[0-9A-Fa-f]+").unwrap();
    match re2.find(state.rest().as_ref()) {
        None => {},
        Some(mat) => {
            let str = mat.as_str();
            match u32::from_str_radix(str, 16) {
                Err(e) => {},
                Ok(z) => {
                    state.advance(str.len());
                    state.push_token(Token::Unsigned(z));
                    return true;
                }
            }
        }
    }

    return false;
}
