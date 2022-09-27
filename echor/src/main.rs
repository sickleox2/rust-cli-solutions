extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Leonardo Mignini <l.mignini@icloud.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("text")
                .required(true)
                .min_values(1)
                .help("Input text")
                .value_name("TEXT"),
        )
        .get_matches();
    // println!("{:#?}", matches);
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    if omit_newline {
        print!("{}", text.join(" "));
        // io::stdout().flush().unwrap();
    } else {
        println!("{}", text.join(" "));
    }
}
