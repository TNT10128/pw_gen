use crate::pw_gen::arg_parsing::Args;
use log::error;
use std::{fs::OpenOptions, io::Write};

pub fn write_passwords_to_file(args: &Args, passwords: &Vec<String>) {
    let file_path = &args.file;

    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                error!("File already exists, canceling: {}", file_path);
            } else {
                error!("Failed to open file: {}", e);
            }
            return;
        }
    };

    for password in passwords {
        match file.write_all(password.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to write to file: {}", e);
                return;
            }
        }
        match file.write_all(b"\n") {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to write to file: {}", e);
                return;
            }
        }
    }
}
