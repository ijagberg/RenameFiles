use clap::{App, Arg};
use std::fs;
use std::path::PathBuf;
use std::result::Result;
use uuid::Uuid;

#[derive(Debug)]
struct Config {
    suppress: bool,
    verbose: bool,
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

    run(config, files);
}

fn run(config: Config, files: Vec<&str>) {
    if !config.suppress {
        // print confirmation
        println!(
            "Are you sure you want to rename {} files? (yes/NO)",
            files.len()
        );
        let user_answer = {
            let mut buffer_string = String::new();
            std::io::stdin()
                .read_line(&mut buffer_string)
                .expect("could not read line");
            buffer_string.trim().to_string()
        };

        if user_answer.to_lowercase() != "yes" {
            return;
        }
    }

    rename_files(config, files);
}

fn rename_files(config: Config, files: Vec<&str>) {
    let mut files_count = 0;
    for file in files {
        if file.starts_with('.') {
            continue;
        }
        // rename single file
        let metadata = fs::metadata(file)
            .unwrap_or_else(|_| panic!("could not retrieve metadata for file: {}", file));
        if metadata.is_file() {
            match rename_file(&config, PathBuf::from(file)) {
                Ok(()) => files_count += 1,
                Err(e) => eprintln!("Error for file {}: {}", file, e),
            }
        }
    }

    if config.verbose {
        println!("Successfully renamed {} files.", files_count);
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
