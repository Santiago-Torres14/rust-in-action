use std::io::BufReader;
use std::{fs, io::BufRead};

use clap::{arg, command};
use regex::Regex;

fn main() {
    let args = command!()
        .about("Searches for patterns")
        .version("0.1")
        .arg(arg!(-p --pattern <pattern> "The pattern to search for"))
        .arg(arg!(-f --file <file> "file to find the pattern"))
        .get_matches();

    let file_arg = args
        .get_one::<String>("file")
        .expect("Not argument was provided for file");

    let pattern_arg = args
        .get_one::<String>("pattern")
        .expect("Not argument was provided for pattern");

    let file = fs::File::open(file_arg).expect("File not found");
    let buf_reader = BufReader::new(file);

    let re = Regex::new(pattern_arg).expect("Pattern expected");

    buf_reader.lines().for_each(|value| {
        if let Ok(line) = value {
            match re.find(&line) {
                Some(result) => println!("offset: {result:?} line: {line}"),
                None => (),
            }
        }
    })
}
