use clap::{command, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Whether uppercase letters should be included in the password.
    #[arg(short, long)]
    pub uppercase: bool,

    /// Whether lowercase letters should be included in the password.
    #[arg(short, long)]
    pub lowercase: bool,

    /// Whether numbers should be included in the password.
    #[arg(short, long)]
    pub numbers: bool,

    /// Whether symbols should be included in the password.
    #[arg(short, long)]
    pub symbols: bool,

    /// The length of the generated password.
    #[arg(long, default_value_t = 16)]
    pub length: i32,

    /// User-specified characters to include in the password.
    #[arg(short, long, default_value_t = String::from(""))]
    pub custom: String,

    /// The amount of passwords to generate with the given settings.
    #[arg(short, long, default_value_t = 1)]
    pub amount: i32,

    /// The file to output the passwords to. Optional.
    #[arg(short, long, default_value_t = String::from(""))]
    pub file: String,
}
