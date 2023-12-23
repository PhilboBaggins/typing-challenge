extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use std::io::{Write, stdout, stdin};
use std::time::Instant;

fn typing_test(target: &str) -> Result<(), std::io::Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    write!(stdout, "Type \"{}\": ", target)?;
    stdout.flush()?;

    let mut mistakes: i32 = 0;
    let mut score: f64 = 0.0;
    let mut multiplier: f64 = 1.0;

    let start_time = Instant::now();

    let mut idx = 0;
    for c in stdin.keys() {
        let input_char = match c {
            Ok(Key::Ctrl('c')) => std::process::exit(0),
            Ok(Key::Char(c)) => c,
            Ok(_) => continue,
            Err(_) => break,
        };

        let target_char = target.chars().nth(idx).unwrap();
        if input_char == target_char {
            score += multiplier * 10.0;
            multiplier *= 1.2;
            write!(stdout, "{}", input_char)?;
            stdout.flush()?;
            idx += 1
        } else {
            mistakes += 1;
            score -= 10.0;
            multiplier = 1.0;
            write!(stdout, "*")?;
            stdout.flush()?;
        }

        if idx == target.len() {
            let time_taken = start_time.elapsed().as_secs_f64();
            let chars_typed: f64 = target.len() as f64;
            let speed = (chars_typed / time_taken) * 60.0; // chars per minute
            write!(stdout, "\r\nWell done!")?;
            write!(stdout, "\r\nIt took you {}{:.2}{} seconds.", color::Fg(color::Green), time_taken, style::Reset)?;
            write!(stdout, "\r\nYour typing speed was {}{:.2}{} characters per minute.", color::Fg(color::Green), speed, style::Reset)?;
            write!(stdout, "\r\nYou scored: {}{:.2}{}", color::Fg(color::Green), score, style::Reset)?;
            write!(stdout, "\r\nMaking {}{}{} mistakes.", color::Fg(color::Red), mistakes, style::Reset)?;
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
        match typing_test(target) {
            Ok(_) => println!("\n"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
