# 🔑 pw_gen
A command-line password generator made in Rust. Also my first Rust program. (warning: code may be horrible, or it might not be, I don't know)

![GNU GPL v3](https://img.shields.io/github/license/TNT10128/pw_gen?style=for-the-badge)
![Codefactor code quality](https://img.shields.io/codefactor/grade/github/TNT10128/pw_gen?style=for-the-badge)

## ℹ️ Description
pw_gen is a command line password generator written in Rust. It allows you to customize the characters to be included in your password, as well as the length and the amount of passwords to generate. Additionally, there is the option to output the generated passwords to a file.

## 🤖 Command usage
```
Usage: pw_gen [OPTIONS]

Options:
  -u, --uppercase        Whether uppercase letters should be included in the password
  -l, --lowercase        Whether lowercase letters should be included in the password
  -n, --numbers          Whether numbers should be included in the password
  -s, --symbols          Whether symbols should be included in the password
      --length <LENGTH>  The length of the generated password [default: 16]
  -c, --custom <CUSTOM>  User-specified characters to include in the password
  -a, --amount <AMOUNT>  The amount of passwords to generate with the given settings [default: 1]
  -f, --file <FILE>      The file to output the passwords to. Optional.
  -h, --help             Print help
  -V, --version          Print version
```

## ❓ How to use
> Before you begin: Make sure you have **Rust** and **Cargo** installed on your system!

Clone the repository and open the folder with a terminal. Type `cargo build` and navigate to the "target/release" folder. The executable should be there. Enjoy!