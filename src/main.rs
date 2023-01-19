mod pw_gen;

use clap::Parser;
use log::{info, error};
use pw_gen::{arg_parsing::Args, files::write_passwords_to_file, generation::generate_password};

fn print_startup_text() {
    let slanted_font = figlet_rs::FIGfont::from_file("fonts/slant.flf").unwrap();
    let figure = slanted_font.convert("pwgen");
    match figure {
        Some(figure) => info!("{}", figure),
        None => error!("Could not print startup text!")
    }
}

fn main() {
    match simple_logger::init() {
        Ok(_) => (),
        Err(err) => println!("Logging error: {}", err)
    }

    print_startup_text();

    let args = Args::parse();
    let password_amount = args.amount;
    let mut passwords: Vec<String> = vec![];

    for i in 0..password_amount {
        let password = generate_password(&args);
        passwords.push(password.to_string());
        info!("({}) Generated password: {}", i + 1, password);
    }

    if !args.file.is_empty() {
        info!("Writing passwords to file.");
        write_passwords_to_file(&args, &passwords);
        info!("Finished.");
    }
}