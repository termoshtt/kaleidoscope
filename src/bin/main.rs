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
                if line.len() == 0 {
                    println!("Empty line, continue. Please exit by Ctrl-D if you want.");
                    continue;
                }
                println!("Input = {}", line);
                let mut p = parser::input();
                if let Ok((ast, _)) = p.parse(line.as_str()) {
                    println!("AST = {:?}", ast);
                    if let Err(e) = ast.codegen(&mut m, &mut ir, &mut st) {
                        println!("Failed to generate LLVM IR");
                        println!("  Error: {}", e.comment);
                        println!("  Trace:");
                        for t in e.trace.iter() {
                            println!("    {:?}", t);
                        }
                        continue;
                    }
                    println!("{}", m.to_string());
                } else {
                    println!("Failed to parse input. continue.");
                    continue;
                }
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
                println!("Unknown Error: {:?}", err);
                break;
            }
        }
        rl.save_history("history.txt").unwrap();
    }
}
