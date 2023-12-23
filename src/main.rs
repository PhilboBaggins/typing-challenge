extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn typing_test(target: &str) -> Result<(), std::io::Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(stdout, "Type \"{}\": ", target)?;
    stdout.flush()?;

    let mut idx = 0;
    for c in stdin.keys() {
        let input_char = match c {
            Ok(Key::Char(c)) => c,
            Ok(_) => continue,
            Err(_) => break,
        };

        let target_char = target.chars().nth(idx).unwrap();
        if input_char == target_char {
            write!(stdout, "{}", input_char)?;
            stdout.flush()?;
            idx += 1
        } else {
            write!(stdout, "*")?;
            stdout.flush()?;
        }

        if idx == target.len() {
            break;
        }
    }

    Ok(())
}

fn main() {
    let targets = [
        "abc",
        "def",
        "ghi",
        // "jkl",
        // "mno",
        // "pqr",
        // "stu",
        // "vwx",
        // "yz",
    ];

    for target in &targets {
        println!("Target: {}", target);
        match typing_test(target) {
            Ok(_) => println!("\n"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
