
pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum Token {
    Keyword(String),
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
