extern crate kaleidscope;
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
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
