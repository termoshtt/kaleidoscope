extern crate combine;
extern crate kaleidscope;
extern crate rustyline;

use combine::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use kaleidscope::*;

fn main() {
    let mut m = llvm::Module::new("myks");
    let mut ir = llvm::IRBuilder::new();
    let mut st = llvm::SymbolTable::new();

    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        eprintln!("No history. Create new one.")
    }
    loop {
        let input = rl.readline("ks> ");
        match input {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("Input = {}", line);
                let mut p = kaleidscope::parser::input();
                let (ast, _) = p.parse(line.as_str()).expect("Cannot parse input");
                println!("AST = {:?}", ast);
                ast.codegen(&mut m, &mut ir, &mut st);
                println!("{}", m.to_string());
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted by user (SIGINT)");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Get EOF. Bye :)");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        rl.save_history("history.txt").unwrap();
    }
}
