use std::{env::args, process::ExitCode, str::FromStr};
use proc_macro2::TokenStream;
mod parse;

fn main() -> ExitCode {
    let mut arg_string = String::new();
    for arg in args().skip(1) {
        arg_string.push_str(&arg);
        arg_string.push(' ');
    }

    match TokenStream::from_str(&arg_string) {
        Ok(tokens) => {
            match parse::ramp(tokens) {
                Ok(s) => {
                    println!("{}", s);
                    ExitCode::SUCCESS
                },
                Err(e) => {
                    eprintln!("Parse Error: {}", e.string);
                    ExitCode::FAILURE
                },
            }
        },
        Err(e) => {
            eprintln!("Parse Error: {e}.");
            ExitCode::FAILURE
        },
    }
}