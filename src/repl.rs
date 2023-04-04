use std::io::{stdin, stdout, Write};

use colored::Colorize;

use crate::{parser::Parser, scanner::Scanner};

pub fn run_repl() {
    println!("{}", "pico-lambda λ".bright_cyan());
    println!(
        "{} {}",
        "welcome distant traveller, to the land of".white(),
        "ℓαмв∂α...".bright_cyan()
    );
    // TODO UnUgly
    println!(
        "{}{}{}",
        "type ".white(),
        "'quit'".red(),
        " to exit the REPL.".white(),
    );

    'repl: loop {
        print!("{} ", "~".bright_green());
        stdout().flush().unwrap();

        let mut input = String::new();
        let stdin = stdin();

        stdin.read_line(&mut input).unwrap();
        if input.contains("quit") {
            println!("{}", "Goodbye and thanks for all the fish ><> ><>".green());
            break 'repl;
        }

        // println!("input: {}", format!("'{}'", input.trim().green()).green());
        let tokens = Scanner::scan_str(&input);
        // println!("tokens: {tokens:?}");
        let ast = Parser::parse_tokens(tokens).unwrap(); // TODO: Handle properly
        println!("  {}", ast.to_string().green())
    }
}
