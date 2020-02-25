
mod text;
mod code;

fn main() {
    let input = "(( ( (   (12423))))0xff)))))";

    let result = text::lexer::lex(input.to_string());
    println!("{:?}", result);
}
