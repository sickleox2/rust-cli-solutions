extern crate core;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};


type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Leonardo Mignini <l.mignini@icloud.com>")
        .about("Rust uniq")
        .args(&[
            Arg::with_name("input")
                .help("Input file")
                .value_name("INPUT")
                .default_value("-")
                .multiple(false),
            Arg::with_name("output")
                .help("Output file")
                .multiple(false)
                .takes_value(true)
                .default_value(""),
            Arg::with_name("count")
                .help("Count repeats of same line")
                .short("c")
                .multiple(false)
                .takes_value(false)
                .long("count"),
        ])
        .get_matches();
    Ok(Config {
        input_file: (matches.value_of("input").unwrap().to_string()),
        output_file: matches.value_of("output").map(String::from),
        count: matches.is_present("count"),
    })
}

fn open(path: &String) -> MyResult<Box<dyn BufRead>> {
    match path.as_str() {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}


pub fn run(args: Config) -> MyResult<()> {
    // line word byte filename
    match open(&args.input_file) {
        Err(err) => panic!("{}: {}", &args.input_file, err),
        Ok(mut file) => {
            
            let mut buffer = String::new();

            file.read_to_string(&mut buffer)?;
            let mut cursor = io::Cursor::new(buffer);
            let mut buf = String::new();
            let mut consecutive  = 0;
            let mut result = Vec::new();
            let mut previous= String::new();
            let mut counter = 0;
            while let Ok(n) = cursor.read_line(&mut buf) {
                if counter == 0 {
                    previous = buf.clone();
                }
                if n == 0 {
                    if consecutive != 0 {
                        if args.count {
                            result.push(format!("   {} {}", consecutive, previous));
                        } else {
                            result.push(previous);
                        }
                    }
                    break;
                }
                if !buf.is_empty() {
                    if previous.trim() != buf.trim() {
                        if args.count {
                            result.push(format!("   {} {}", consecutive, previous));
                        } else {
                            result.push(previous);
                        }
                        consecutive = 1;
                    }
                    else {
                        consecutive += 1;
                    }
                }
                previous = buf.clone();
                buf.clear();
                counter += 1
            }
            let out_path = args.output_file.unwrap();
            if out_path != *"" {
                    let mut output_file = File::create(out_path)?;
                for rep in result {
                    write!(&mut output_file, "{}", rep)?;
                }
            } else {
                for rep in result {
                    print!("{}", rep);
                }
            }
        }
    }
    Ok(())
}
