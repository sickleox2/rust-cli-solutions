use clap::{App, Arg};
use std::error::Error;
use std::fs::{read, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Leonardo Mignini <l.mignini@icloud.com>")
        .about("Rust head")
        .args(&[
            Arg::with_name("input")
                .help("Input files")
                .value_name("INPUT")
                .default_value("-")
                .multiple(true),
            Arg::with_name("lines")
                .help("Number of lines to be shown")
                .short("n")
                .multiple(false)
                .value_name("LINES")
                .default_value("10")
                .long("lines"),
            Arg::with_name("bytes")
                .help("Number of bytes to be shown")
                .short("c")
                .takes_value(true)
                .multiple(false)
                .value_name("BYTES")
                .conflicts_with("lines")
                .long("bytes"),
        ])
        .get_matches();
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("input").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

fn open(path: &str) -> MyResult<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}
pub fn run(args: Config) -> MyResult<()> {
    // println!("{:#?}", args);
    let files_num = args.files.len();
    let mut file_counter = 1;
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if files_num > 1 {
                    println!("==> {} <==", &filename);
                }
                if args.bytes != None {
                    let mut reader = BufReader::new(file);
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes)?;
                    if !bytes.is_empty() {
                        print!("{}", String::from_utf8_lossy(&bytes[..args.bytes.unwrap()]));
                    }
                } else {
                    let mut buffer = String::new();

                    file.read_to_string(&mut buffer)?;
                    let mut cursor = io::Cursor::new(buffer);
                    let mut buf = String::new();
                    let mut counter  = 0;
                    while let Ok(n) = cursor.read_line(&mut buf) {
                        if counter == args.lines {
                            break;
                        }
                        if n == 0 {
                            break;
                        }
                        if !buf.is_empty() {
                            print!("{}", buf);
                        }
                        buf.clear();
                        counter += 1;
                    }
                }
                if files_num > 1 && file_counter != files_num {
                    println!();
                    file_counter += 1;
                }
            }
        }
    }

    Ok(())
}
