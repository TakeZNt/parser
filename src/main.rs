mod lexer;

fn main() {
    let result = lexer::lex("1 + 2 * 3 - -10");
}
