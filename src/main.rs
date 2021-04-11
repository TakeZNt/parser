//mod interpreter;
mod lexer;
mod parser;
mod compiler;

//use interpreter::Interpreter;
use compiler::RpnCompiler;
use parser::Ast;

use std::error::Error;
use std::io;

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    //let mut interpreter = Interpreter::new();
    let mut compiler = RpnCompiler::new();

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

                // 構文解析
                let ast = match line.parse::<Ast>() {
                    Ok(ast) => ast,
                    Err(e) => {
                        e.show_diagnostic(&line);
                        show_trace(e);
                        continue;
                    }
                };

                // 評価
                // let n = match interpreter.eval(&ast) {
                //     Ok(n) => n,
                //     Err(e) => {
                //         e.show_diagnostic(&line);
                //         show_trace(e);
                //         continue;
                //     }
                // };
                let rpn = compiler.compile(&ast);

                println!("{}", rpn);
            }
        } else {
            break;
        }
    }
}

fn show_trace<E: Error>(e: E) {
    eprintln!("{}", e);
    let mut source = e.source();
    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source();
    }
}
