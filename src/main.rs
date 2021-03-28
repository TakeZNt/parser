use std::io;
mod lexer;

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();

        if let Some(Ok(line)) = lines.next() {
            if line.len() > 0 {
                if line == "exit" || line == "quit" {
                    prompt("bye.").unwrap();
                    break;
                }
                let token = lexer::lex(&line);
                println!("{:?}", token);
            }
        } else {
            break;
        }
    }
}
