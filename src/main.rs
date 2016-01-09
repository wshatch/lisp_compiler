#[macro_use]
extern crate nom;

mod nom_parse_funcs;
mod operations;
mod ast;
mod token;
use std::io;
use std::io::Write;
use std::process;

fn main(){
  loop {
    print!("lispy>");
    io::stdout().flush();
    handle_input(get_input().trim());
  }
}

fn get_input() -> String {
  let reader = io::stdin();
  let mut buffer = String::new();
  reader.read_line(&mut buffer).unwrap();
  buffer
}

fn handle_input(input: &str){
  match input{
    "exit" => process::exit(0),
    _ => println!("{}", input)
  }
}

