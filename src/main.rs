use std::fs::read_to_string;
use std::io::{stdin, stdout};
use std::process::exit;

use clap::{App, Arg};

use bfk::*;

fn main() {
    fn is_usize(v: String) -> Result<(), String> {
        match v.parse::<usize>() {
            Ok(_) => Ok(()),
            Err(_) => Err("Must be a positive integer".into())
        }
    }

    let matches = App::new("bf")
        .version("0.2.0")
        .author("Yusaku Hashimoto <nonowarn@gmail.com>")
        .about("Brainfuck Interpreter")
        .arg(
            Arg::with_name("PROGRAM")
                .help("Brainfuck program to run")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("no_compress")
                .help("Don't compress operations before running")
                .short("n")
                .long("no-compress")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("language")
                .help("Language to run as a string concatenated with instructions in order of: +-><,.[]")
                .short("l")
                .long("language")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("buffer_size")
                .help("Tape buffer size in bytes")
                .short("b")
                .long("buffer-size")
                .takes_value(true)
                .validator(is_usize)
        )
        .get_matches();

    let filename = matches.value_of("PROGRAM").unwrap();
    let buffer_size = match matches.value_of("buffer_size") {
        None => { 1024 * 1024 }
        Some(size) => { size.parse().expect("Positive integer") }
    };

    let no_compress = matches.is_present("no_compress");

    let code = match read_to_string(filename) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error while reading {}: {}", filename, err);
            exit(exitcode::NOINPUT);
        }
    };

    let language = match matches.value_of("language") {
        Some(language_str) => match Language::make_from_string(&language_str.to_string()) {
            None => {
                eprintln!("language must have exact 8 characters");
                exit(exitcode::DATAERR);
            }
            Some(language) => language
        },
        None => Language::default(),
    };

    let ops = parse(&code, &language);

    let mut data = vec![0u8; buffer_size];

    let mut stdout = stdout();
    let mut stdin = stdin();

    let mut env = Environment::new(&mut data, &mut stdin, &mut stdout);

    if no_compress {
        run(&ops, &mut env);
    } else {
        let compressed_ops = compress(&ops);
        run(&compressed_ops, &mut env);
    }

}
