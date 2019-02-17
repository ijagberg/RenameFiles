extern crate clap;
use clap::{App, Arg};
use std::fs;

// program
//     .option('-g, --guid', 'Use the GUID naming convention (####-####-####-####, ...). This is the default naming convention used.')
//     .option('-i, --incremental', 'Use the incremental naming convention (1, 2, 3, ...)')
//     .option('-s, --suppress', 'Suppress prompt for confirmation')
//     .parse(process.argv);

struct Config {
    suppress: bool,
}

fn main() {
    let matches = App::new("RenameFiles")
        .version("1.0")
        .author("Isak Jägberg <ijagberg@gmail.com>")
        .about("Tool for renaming directories")
        .arg(
            Arg::with_name("suppress")
                .short("s")
                .help("Suppress prompt for confirmation"),
        )
        .get_matches();

    let config = Config{
        suppress: matches.is_present("suppress")
    };

    
}