use regex::Regex;
use std::str::FromStr;

// The lexer consumes a stream of Unicode characters and produces a stream of tokens that represent lexemes. Whitespace tokens are ignored on their own.
// Unicode characters may only appear in string literals and comments; the grammar is otherwise formed from UTF-8 characters.
// https://webassembly.github.io/spec/core/text/lexical.html#
// The lexical format of tokens are specified by regular languages which admit the following production rules: terminals and nonterminals
// A -> a, A -> aB

// The next token is taken to be the longest possible sequence of characters defined by the grammar.

type Lex<A> = Result<(A, usize), String>;

pub enum Token {
    Keyword(Keyword),
    // TODO: Are 32-bit types sufficient here?
    Unsigned(u32),
    Signed(i32),
    Floating(f32),
    String(),
    OpenParen,
    CloseParen,
    Reserved(Reserved)
}

pub struct Keyword {

}

pub struct Reserved {

}

fn lex_token(input: &str) -> u32 {
    234
}

fn lex_keyword(input: &str) -> u32 {
    234
}

fn lex_reserved(input: &str) -> u32 {
    34
}

// TODO: Use lazy_static to avoid compiling the regex on every invocation
// TODO: The regex should admit underscores
fn lex_unsigned(input: &mut String) -> Lex<u32> {
    let a1 = lex_dec(input);
    if a1.is_ok() {
        input.replace_range(..a1.unwrap().1, "");
        return a1;
    }

    let a2 = lex_hex(input);
    if a2.is_ok() {
        return a2;
    }

    return Err("Failed to parse unsigned integer.".into());
}


fn lex_dec(input: &String) -> Lex<u32> {
    let re = Regex::new(r"^\d+").unwrap();
    let mat = re.find(input).ok_or("Invalid u32")?;
    let str = mat.as_str();
    let z = u32::from_str_radix(str, 10).map_err(|_| "Invalid u32.".to_string())?;
    return Ok((z, str.len()));
}

fn lex_hex(input: &String) -> Lex<u32> {
    let re = Regex::new(r"^0x[0-9A-Fa-f]+").unwrap();
    let mat = re.find(input).ok_or("Invalid u32")?;
    let str = mat.as_str();
    let z = u32::from_str_radix(str, 16).map_err(|_| "Invalid u32.".to_string())?;
    return Ok((z, str.len()));
}
