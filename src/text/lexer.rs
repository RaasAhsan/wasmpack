use regex::Regex;
use std::str::FromStr;

// The lexer consumes a stream of Unicode characters and produces a stream of tokens that represent lexemes. Whitespace tokens are ignored on their own.
// Unicode characters may only appear in string literals and comments; the grammar is otherwise formed from UTF-8 characters.
// https://webassembly.github.io/spec/core/text/lexical.html#
// The lexical format of tokens are specified by regular languages which admit the following production rules: terminals and nonterminals
// A -> a, A -> aB

// The next token is taken to be the longest possible sequence of characters defined by the grammar.

// How to express alternatives of a production more effectively?

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

pub enum Token {
    Keyword(Keyword),
    // TODO: Are 32-bit types sufficient here?
    Unsigned(u32),
    Signed(i32),
    Float(f32),
    String(String),
    Id(String),
    OpenParen,
    CloseParen,
    Reserved(String)
}

pub struct Keyword {

}

fn lex(input: String) -> Vec<Token> {
    let mut state = State {
        input: input.clone(),
        cursor: 0,
        tokens: vec![]
    };

    state.tokens
}

fn lex_token(state: &mut State) -> bool {
    return if state.rest().starts_with("(") {
        state.advance(1);
        state.push_token(Token::OpenParen);
        true
    } else if state.rest().starts_with(")") {
        state.advance(1);
        state.push_token(Token::CloseParen);
        true
    } else if state.rest().starts_with("$") {
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
fn lex_unsigned(state: &mut State) -> bool {
    let re = Regex::new(r"^\d+").unwrap();
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
