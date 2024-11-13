extern crate core;

use clap::{Parser};
use crate::lexer::lex;
use crate::utils::{fs, repl};

mod lexer;
mod parser;
mod token;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "Code Like Essay")]
#[command(author = "Ijusthahaha")]
#[command(about = "Code Like Essay REPL")]
#[command(version)]
struct Cli {
    command: Option<String>
}

fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command.as_deref() {
        fs(command);
        return
    }
    repl("Code Like Essay", "");
}

#[test]
fn test() {
    let vec = lex("set var1 to a. display 1 add 2. suppose that a equals to 2 or 3, display true. Otherwise, display false.");
    println!("{:?}", vec);

    let vec = lex("display a greater than b.");
    println!("{:?}", vec);

    let vec = lex("loop until a equals to b, display 'running', decrease a by 1");
    println!("{:?}", vec);

    let vec = lex("set a to a list of 1,2,  and 3.");
    println!("{:?}", vec);

    let vec = lex("set myMap to a map of 'name' equals 'Alice', 'age' as 30");
    println!("{:?}", vec);
}