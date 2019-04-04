extern crate clap;
extern crate uuid;
use clap::{App, Arg};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::result::Result;
use uuid::Uuid;

// program
//     .option('-s, --suppress', 'Suppress prompt for confirmation')
//     .parse(process.argv);

struct Config {
    suppress: bool,
    verbose: bool,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Suppress: {}, Verbose: {}", self.suppress, self.verbose)
    }
}

fn main() {
    // handle command line parsing
    let matches = App::new("RenameFiles")
        .version("1.0")
        .author("Isak Jägberg <ijagberg@gmail.com>")
        .about("Tool for renaming directories")
        .arg(
            Arg::with_name("suppress")
                .short("s")
                .long("suppress")
                .help("Suppress prompts for confirmation"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("--verbose")
                .help("Display verbose output of renamings"),
        )
        .arg(
            Arg::with_name("files")
                .required(true)
                .min_values(1)
                .help("Files to rename"),
        )
        .get_matches();

    let config = Config {
        suppress: matches.is_present("suppress"),
        verbose: matches.is_present("verbose"),
    };
    let files: Vec<&str> = matches.values_of("files").unwrap().collect();
    println!("{}", config);

    run(config, files);
}

fn run(config: Config, files: Vec<&str>) {
    if !config.suppress {
        // print confirmation
        println!("This will be a confirmation eventually");
        return;
    }

    rename_files(config, files);
}

fn rename_files(config: Config, files: Vec<&str>) {
    for file in files {
        if file.starts_with('.') {
            continue;
        }
        // rename single file
        let metadata =
            fs::metadata(file).expect(&format!("Could not retrieve metadata for file: {}", file));
        if metadata.is_file() {
            match rename_file(&config, PathBuf::from(file)) {
                Ok(()) => {}
                Err(e) => eprintln!("Error for file {}: {}", file, e),
            }
        }
    }
}

fn rename_file(config: &Config, current_path: PathBuf) -> Result<(), std::io::Error> {
    let random_name = Uuid::new_v4().to_string();
    let mut new_path = PathBuf::from(current_path.parent().unwrap());
    new_path.push(random_name);
    new_path.set_extension(current_path.extension().unwrap());
    match fs::rename(&current_path, new_path.clone()) {
        Ok(()) => {
            if config.verbose {
                println!(
                    "{} -> {}",
                    current_path.to_str().unwrap(),
                    new_path.to_str().unwrap()
                );
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
