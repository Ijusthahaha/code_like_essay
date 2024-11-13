
use std::fs;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub fn fs(command: &str) {
    let c = match fs::read_to_string(&command) {
        Ok(content) => content,
        Err(error) => panic!("{}", error),
    };
    c.split(".").for_each(|x| {
        if x != "\n" {
            println!("{}", x);
        }
    });
}
pub fn repl(name: &str, version: &str) {
    let mut rl = DefaultEditor::new().unwrap();
    println!("{} {}", name, version);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => println!("Line: {:?}", line),
            Err(ReadlineError::Interrupted) => {
                println!("Goodbye.");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}