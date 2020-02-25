
mod text;
mod code;

fn main() {
    let input = "(( ( (   (12423))))0xff)))$a) hello-world)";

    let result = text::lexer::lex(input.to_string());
    println!("{:?}", result);
}
