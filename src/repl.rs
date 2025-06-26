use std::io::{self, Write};
use std::process::exit;

use crate::lexer::Lexer;
use crate::token::TokenType;

const HELP_MESSAGE: &str = "
help:  Print this help message
exit:  Exit the REPL";

pub struct Repl;

impl Repl {
    pub fn start() {
        print_welcome();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let input: String = read_input();
            if !handle_command(&input) {
                handle_input(input);
            }
        }
    }
}

fn read_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn print_welcome() {
    println!("Welcome to WMLang v{}", env!("CARGO_PKG_VERSION"));
    println!("Type \"help\" for more information");
}

fn handle_command(input: &String) -> bool {
    match input.trim() {
        "help" => {
            println!("{}", HELP_MESSAGE.trim());
        }
        "exit" => {
            exit(0);
        }
        _ => return false,
    }
    true
}

fn handle_input(input: String) {
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        println!("({:?}, {})", token.ttype, token.literal);
        if token.ttype == TokenType::Eof {
            break;
        }
    }
}
