use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::process;
use ansi_term::Colour::{Green, Yellow, Red};

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
  check_input_file();
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn check_input_file() {
  let filename: &str = "words.txt";
  
  // false => permission error, doesn't exist
  if !Path::new(filename).exists() {
    println!("{}", Yellow.paint("The input file doesnt exist, attempting to create one!"));

    if let Ok(result) = create_default_file(filename) {
      println!("Matched {:?}!", result);
      println!("{}", Green.paint("Done!"));
    } else {
      println!("{}", Red.paint("We were not able to create an input file.\nThough, you may create it by yourself."));
      process::exit(1);
    }
  }

  // someone will probably have more than 255 words, so u16 is better
  let mut line_number: u16 = 0;

  if let Ok(lines) = read_lines(filename) {
    // Consumes the iterator, returns an (Optional) String
    for line in lines {
      if let Ok(data) = line {
        println!("{0}: {1}", line_number, data);
      }
      line_number = line_number + 1;
    }
  }
}