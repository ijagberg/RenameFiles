extern crate clap;
extern crate uuid;
use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;
use std::result::Result;
use uuid::Uuid;

// program
//     .option('-s, --suppress', 'Suppress prompt for confirmation')
//     .parse(process.argv);

struct Config {
    suppress: bool,
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
                .help("Suppress prompt for confirmation"),
        )
        .arg(Arg::with_name("files").required(true).min_values(1))
        .get_matches();

    let config = Config {
        suppress: matches.is_present("suppress"),
    };
    let files: Vec<&str> = matches.values_of("files").unwrap().collect();

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

fn rename_files(_config: Config, files: Vec<&str>) {
    for file in files {
        if file.starts_with('.') {
            continue;
        }
        // rename single file
        let metadata = match fs::metadata(file) {
            Ok(metadata) => metadata,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        if metadata.is_file() {
            match rename_file(PathBuf::from(file)) {
                Ok(()) => {}
                Err(e) => eprintln!("Error for file {}: {}", file, e),
            }
        }
    }
}

fn rename_file(current_path: PathBuf) -> Result<(), std::io::Error> {
    let random_name = Uuid::new_v4().to_string();
    let mut new_path = PathBuf::from(current_path.parent().unwrap());
    new_path.push(random_name);
    new_path.set_extension(current_path.extension().unwrap());
    match fs::rename(&current_path, new_path.clone()) {
        Ok(()) => {
            println!(
                "{} -> {}",
                current_path.to_str().unwrap(),
                new_path.to_str().unwrap()
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}
