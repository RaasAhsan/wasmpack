use crate::text::Token;
use crate::ast::module::Module;

// The parser consumes a stream of tokens produced by the lexer and produces an abstract syntax tree of S-expressions representing the WebAssembly module.
// Implemented as a top-down, recursive descent, backtracking parser.

// TODO: Define an enum error hierarchy
type Parse<A> = Option<A>;

struct State {
    tokens: Vec<Token>,
    cursor: usize
}

impl State {



}

pub fn parse(tokens: Vec<Token>) {
    let state = State {
        tokens,
        cursor: 0
    };

    parse_module(&state);
}


// Rust's ? operator is perfect for parsing sequences. How do we parse alternatives? Need to reset the cursor.
fn parse_module(state: &State) -> Parse<Module> {
    parse_left_paren(state)?;
    parse_right_paren(state)?;
    parse_eof(state)?;
    None
}

fn parse_left_paren(state: &State) -> Parse<()> {
    None
}

fn parse_right_paren(state: &State) -> Parse<()> {
    None
}

fn parse_eof(state: &State) -> Parse<()> {
    if state.tokens.len() == 0 {
        Some(())
    } else {
        None
    }
}
