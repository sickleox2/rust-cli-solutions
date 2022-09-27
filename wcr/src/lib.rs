use clap::{App, Arg};
use std::error::Error;
use std::fs::{File};
use std::io::{self, BufRead, BufReader, Read};
use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;


pub struct Config {
    files: Vec<String>,
    lines: bool,
    bytes: bool,
    characters: bool,
    words: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Leonardo Mignini <l.mignini@icloud.com>")
        .about("Rust wc")
        .args(&[
            Arg::with_name("input")
                .help("Input files")
                .value_name("INPUT")
                .default_value("-")
                .multiple(true),
            Arg::with_name("lines")
                .help("Display the number of lines")
                .short("l")
                .multiple(false)
                .takes_value(false)
                .long("lines"),
            Arg::with_name("bytes")
                .help("Display the number of bytes")
                .short("c")
                .multiple(false)
                .takes_value(false)
                .long("bytes"),
            Arg::with_name("characters")
                .help("Display the number of characters")
                .short("m")
                .multiple(false)
                .takes_value(false)
                .long("chars")
                .conflicts_with("bytes"),
            Arg::with_name("words")
                .help("Display the number of words")
                .short("w")
                .multiple(false)
                .takes_value(false)
                .long("words"),
        ])
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("input").unwrap(),
        lines: matches.is_present("lines"),
        bytes: matches.is_present("bytes"),
        words: matches.is_present("words"),
        characters: matches.is_present("characters"),
    })
}

fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

pub fn run(args: Config) -> MyResult<()> {
    struct Data {
        current: usize,
        total: usize
    }
    impl Data {
        fn add_total (&mut self) {
            self.total += self.current;

        }
        fn update_current (&mut self, new_current: usize) {
            self.current = new_current;
        }
    }
    let mut bytes = Data {
        current: 0,
        total: 0,
    };
    let mut chars = Data {
        current: 0,
        total: 0,
    };
    let mut words = Data {
        current: 0,
        total: 0,
    };
    let mut lines = Data {
        current: 0,
        total: 0,
    };
    for (file_num, filename) in args.files.iter().enumerate() {
        // line word byte filename
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;
                bytes.update_current(buf.len());
                words.update_current(buf.split_whitespace().count());
                chars.update_current(buf.chars().count());
                lines.update_current(buf.lines().count());
                bytes.add_total();
                words.add_total();
                chars.add_total();
                lines.add_total();
                let mut result = Vec::new();
                let mut totals = Vec::new();
                if args.lines {
                    result.push(lines.current);
                    totals.push(lines.total)
                }
                if args.words {
                    result.push(words.current);
                    totals.push(words.total)
                }
                if args.bytes {
                    result.push(bytes.current);
                    totals.push(bytes.total)
                }
                if args.characters {
                    result.push(chars.current);
                    totals.push(chars.total)
                }
                if filename != "-" {
                if result.is_empty() {
                    println!("{:8}{:8}{:8} {}", lines.current, words.current, bytes.current, filename);
                    if file_num + 1 == args.files.len() && args.files.len() > 1 {
                        println!("{:8}{:8}{:8} total", lines.total, words.total, bytes.total);
                    }
                } else {
                    for arg in result {
                        print!("{:8}", arg);
                    }
                    println!(" {}", filename);
                    if file_num + 1 == args.files.len() && args.files.len() > 1 {
                    for total in totals {
                        print!("{:8}", total);
                        
                        
                    }
                        println!(" total");}

                }} else if result.is_empty() {
                    println!("{:8}{:8}{:8}", lines.current, words.current, bytes.current);
                    if file_num + 1 == args.files.len() && args.files.len() > 1 {
                        println!("{:8}{:8}{:8} total", lines.total, words.total, bytes.total);
                    }
                } else {
                    for arg in result {
                        print!("{:8}", arg);
                        
                    }
                    if file_num + 1 == args.files.len() && args.files.len() > 1 {
                    for total in totals {
                        print!("{:8}", total);
                    }
                    println!(" total");}
                    
                    }
            }
        }
    }
    Ok(())
}
