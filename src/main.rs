use std::fs::read_to_string;
use std::io::{stdin, stdout};

use clap::{App, Arg};

use bf::*;

fn main() {
    let matches = App::new("bf")
        .version("0.1")
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
                .short("s")
                .long("language")
                .takes_value(true)
        )
        .get_matches();

    let filename = matches.value_of("PROGRAM").unwrap();

    let no_compress = matches.is_present("no_compress");

    let code = read_to_string(filename).unwrap();
    let language = match matches.value_of("language") {
        Some(language_str) => Language::make_from_string(&language_str.to_string()),
        None => Language::default(),
    };

    let ops = parse(&code, &language);

    let mut data = [0u8; 1024 * 1024];

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
