extern crate clap;
use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("RenameFiles")
        .version("1.0")
        .author("Isak Jägberg <ijagberg@gmail.com>")
        .about("Tool for renaming directories")
        .get_matches();

    

}
