use crate::pw_gen::arg_parsing::Args;
use log::error;
use rand::seq::SliceRandom;
use std::process::exit;

const UPPERCASE_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "01234569789";
const SYMBOLS: &str = "!@#$%";

fn get_random_element<T>(vec: &Vec<T>) -> &T {
    let chosen = vec.choose(&mut rand::thread_rng());
    match chosen {
        Some(result) => result,
        None => panic!("Couldn't get random element from list!")
    }
}

fn get_available_characters(args: &Args) -> Vec<char> {
    let mut available: Vec<char> = vec![];
    if args.uppercase {
        available.extend(UPPERCASE_LETTERS.chars());
    }
    if args.lowercase {
        available.extend(LOWERCASE_LETTERS.chars());
    }
    if args.numbers {
        available.extend(NUMBERS.chars());
    }
    if args.symbols {
        available.extend(SYMBOLS.chars())
    }
    available.extend(args.custom.chars());

    available
}

pub fn generate_password(args: &Args) -> String {
    let mut generated_password = String::from("");
    let available_characters = get_available_characters(args);

    if available_characters.is_empty() {
        error!("Invalid usage. Run with --help for more information.");
        exit(1);
    }

    for _ in 0..args.length {
        generated_password.push(*get_random_element(&available_characters));
    }

    generated_password
}