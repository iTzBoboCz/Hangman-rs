use std::io::{ self, Write, BufRead, BufReader };
use std::str;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::process;
use std::thread;
use termion::{ raw::IntoRawMode, input::TermRead };
use std::time::Duration;
use ansi_term::Colour::{ Green, Yellow, Red };
use stringsort::insertsort;

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

struct GameData {
  won: usize,
  lost: usize
}

fn main() {
  let words = load_input_file();

  // default to 5 lives
  let lives: usize = 5;

  let mut gamedata: GameData = GameData {
    won: 0,
    lost: 0
  };
  
  for word in words {
    game_screen(&word, lives, &mut gamedata);
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
#[allow(dead_code)]
fn input(message: &'_ impl fmt::Display) -> String {
  let mut string = String::new();

  print!("{}", message);
  // stdout is line-buffered (and print doesn't emit a newline)
  io::Write::flush(&mut io::stdout()).expect("[ERROR] Flush failed!");

  io::stdin().read_line(&mut string).expect("[ERROR] Failed to read from stdin");

  // remove new line at the end
  let ret = String::from(string.trim());
  
  ret
}

fn input_char(message: &str, guessed: String) -> char {
  print!("{}", message);
  // stdout is line-buffered (and print doesn't emit a newline)
  io::Write::flush(&mut io::stdout()).expect("[ERROR] Flush failed!");

  let terminal = io::stdout().into_raw_mode();
  let stdout = terminal.unwrap();

  // Use asynchronous stdin
  let mut stdin = termion::async_stdin().keys();
  let s: char;

  loop {
    // Read input (if any)
    let input = stdin.next();

    // If a key was pressed
    if let Some(Ok(key)) = input {
      match key {
        // Exit if 'ctrl + c' is pressed
        termion::event::Key::Ctrl('c') => {
          s = '\0';
          break;
        },

        // Else print the pressed key
        _ => {
          if let termion::event::Key::Char(k) = key {
            if !guessed.contains(k) {
              s = k;
              stdout.lock().flush().unwrap();
              break;
            }
          }
        }
      }
    }
    thread::sleep(Duration::from_millis(0));
  }

  if s == '\0' {
    println!("\nExitting!");
    process::exit(1);
  }

  s
}

fn clear() {
  print!("\x1B[2J\x1B[1;1H");
}

fn game_screen(word: &str, mut lives: usize, gamedata: &mut GameData) {
  let mut guessed: String = String::new();
  let mut guess: char;
  // let mut info: String = String::from("");
  
  loop {
    clear();

    // println!("{}", info);
    println!("won: {0} | lost: {1}", gamedata.won, gamedata.lost);
    println!("{}", str::repeat("♥", lives));
    print_hangman(lives);
    println!("guessed: {}\n", insertsort(&guessed));
    let output = output_word(word, &guessed);

    println!("{}", output);

    if output == word {
      gamedata.won += 1;
      break;
    } else if lives == 0 {
      gamedata.lost += 1;
      break;
    }

    // check if is one letter and guessed.append(input)
    guess = input_char(&": ", String::from(&guessed));
    if !word.contains(guess) {
      lives -= 1;
    }
    guessed.push(guess);
  }
}

fn output_word(word: &str, guessed: &str) -> String {
  let mut output = String::from("");

  for letter in word.chars() {
    if guessed.contains(letter) || !letter.is_alphabetic() {
      output += &String::from(letter);
    } else {
      output += "_";
    }
  }

  output
}

fn print_hangman(lives: usize) {
  match lives {
    0 => {
      println!(" _________   ");
      println!("|         |  ");
      println!("|         XO ");
      println!("|        /|\\ ");
      println!("|        / \\ ");
      println!("|            ");
      println!("|            ");
    },

    1 => {
      println!(" _________   ");
      println!("|         |  ");
      println!("|         O  ");
      println!("|        /|\\ ");
      println!("|        / \\ ");
      println!("|        ||| ");
      println!("|        ||| ");
    },

    2 => {
      println!(" _________   ");
      println!("|            ");
      println!("|         O  ");
      println!("|        /|\\ ");
      println!("|        / \\ ");
      println!("|        ||| ");
      println!("|        ||| ");
    },

    3 => {
      println!(" _________   ");
      println!("|            ");
      println!("|            ");
      println!("|         O  ");
      println!("|        /|\\ ");
      println!("|        / \\ ");
      println!("|        ||| ");
    },

    4 => {
      println!(" _________   ");
      println!("|            ");
      println!("|            ");
      println!("|            ");
      println!("|         O  ");
      println!("|        /|\\ ");
      println!("|        / \\ ");
    },

    _ => {
      println!("             ");
      println!("             ");
      println!("             ");
      println!("             ");
      println!("          O  ");
      println!("         /|\\ ");
      println!("         / \\ ");
    },
  }
}