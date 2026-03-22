mod store;
mod parser;
mod command;

use std::io::{self, Write};
use store::Store;
use parser::parse;
use command::Command;

fn main() {
    let mut store = Store::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parse(&input) {
            Command::Set(key, value) => {
                store.set(key, value);
                println!("OK");
            }
            Command::Get(key) => match store.get(&key) {
                Some(val) => println!("{}", val),
                None => println!("Key not found"),
            },
            Command::Del(key) => {
                if store.del(&key) {
                    println!("Deleted");
                } else {
                    println!("Key not found");
                }
            }
            Command::Exit => break,
            Command::Invalid => println!("Invalid command"),
        }
    }
}