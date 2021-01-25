use std::io::{ self, Write, BufRead, BufReader };
use std::str;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::process;
use ansi_term::Colour::{ Green, Yellow, Red };

#[allow(dead_code)]
enum Difficulty {
  Easy,
  Medium,
  Hard
}

#[allow(dead_code)]
enum Type {
  Words,
  Sentences
}

fn main() {
  let words = load_input_file();
  
  // default to 5 lives
  let lives: usize = 5;
  
  for word in words {
    let guessed: Vec<char>;

    game_screen(&word, lives, guessed);
  }
}

fn create_default_file(filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    
    // default words
    let words = "\
    forest\n\
    pancake\n\
    castle\n\
    rabbit\n\
    fire\n\
    hippopotamus\n\
    rhinoceros\n\
    giraffe\n";

    file.write_all(words.as_bytes())?;
    Ok(())
}

fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf.lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

fn load_input_file() -> Vec<String> {
  let filename: &str = "words.txt";

  check_input_file(filename);
  read_lines(filename)
}

fn check_input_file(filename: &str) {
  // false => permission error, doesn't exist
  if !Path::new(filename).exists() {
    println!("[INFO] {}", Yellow.paint("The input file doesn't exist, attempting to create one!"));

    if let Ok(result) = create_default_file(filename) {
      println!("Matched {:?}!", result);
      println!("[INFO] {}", Green.paint("Done!"));
    } else {
      println!("[INFO] {}", Red.paint("We were not able to create an input file.\nThough, you may create it by yourself."));
      process::exit(1);
    }
  }
}

// https://users.rust-lang.org/t/why-is-it-so-difficult-to-get-user-input-in-rust/27444/3
fn input(message: &'_ impl fmt::Display) -> String
{
  let mut string = String::new();

  print!("{}", message);
  // stdout is line-buffered (and print doesn't emit a newline)
  io::Write::flush(&mut io::stdout()).expect("[ERROR] Flush failed!");

  io::stdin().read_line(&mut string).expect("[ERROR] Failed to read from stdin");

  // remove new line at the end
  let ret = String::from(string.trim());
  
  ret
}