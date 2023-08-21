/**
 * Author: Eshaan Joshi, evj@andrew.cmu.edu
 *
 * This project is really to just practice my programming and work on something that interested me. I'd always wanted to write an interpreter, and this was an excuse. So what exactly *is* this project?
 * Welcome to Fillet, a programming language written in Rust. This project is based on the book, Crafting Interpreters, from which I've gotten most of my knowledge.
 *
 * Note: At the time of writing this code, I had zero familiarity with Rust. I wanted to learn the language and dive into a project I found interesting, and translating the Java/C heavy book to Rust felt interesting.
 *
 */
//#![allow(non_camel_case_types())]


#[macro_use]
extern crate lazy_static;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod token_handler;
use std::env;
use std::fs;
use std::io::Write;


fn _run(source: String) {
    println!("Input: {input}", input = source);
    let mut tokenized:Vec<token_handler::token> = token_handler::scan_tokens(source);
    token_handler::print_token_list(&mut tokenized);
}

fn _run_prompt() {
    println!("Running Prompt");
    loop {
        print!(">>>");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        println!("");
        _run(line);
    }
}

fn _run_file(filename: String) {
    println!("Running file: {filename}", filename = filename);
    let contents = fs::read_to_string(filename).expect("File Not Found");

    println!("Contents: {cont}", cont = contents);
    _run(contents);
}

fn cmdline_check() -> bool {
    let argument: Vec<String> = env::args().collect();
    if argument.len() > 2 {
        println!("usage: fillet [filename]");
        return false;
    }
    if argument.len() == 1 {
        _run_prompt();
    }
    if argument.len() == 2 {
        let filename_opt = argument.get(1);
        let filename = filename_opt.unwrap().to_string();
        _run_file(filename);
    }
    return true;
}

fn main() {
    cmdline_check();
}
