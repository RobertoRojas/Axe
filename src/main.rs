use clap::{Arg, App, SubCommand};

#[cfg(not(debug_assertions))]
use config::VERSION;

#[cfg(debug_assertions)]
pub const VERSION: &str = "v0.0.0-debug";

use std::path::Path;
use std::process;

fn writeFile(file: &str, data: &[u8]) {

}

fn readFile(file: &str) {

}

fn join(file: &str, count: u8) {

}

fn meld(files: Vec<&str>, output: &str) {

}

fn main() {
    let mut app = App::new("Axe")
        .version(VERSION)
        .author("Roberto Rojas")
        .about("Axe is a command-line interface (CLI) tool that enables users to cut files into sections and subsequently meld them into a single file.")
        .subcommand(SubCommand::with_name("CUT")
            .about("Cuts a file into smaller files")
            .arg(Arg::with_name("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Specifies the path of the file to cut.")
                .takes_value(true)
                .required(true)
                .validator(|v| {
                    if v.trim().is_empty() {
                        Err(String::from("The file path must not be empty."))
                    } else {
                        Ok(())
                    }
                }))
            .arg(Arg::with_name("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Specifies how many files to create from the file.")
                .takes_value(true)
                .default_value("2")
                .validator(|v| {
                    v.parse::<u8>()
                        .map_err(|_| String::from("The count must be a valid integer."))
                        .and_then(|val| {
                            if val < 2 {
                                Err(String::from("The count must be at least 2."))
                            } else {
                                Ok(())
                            }
                        })
                })))
        .subcommand(SubCommand::with_name("MELD")
            .about("Melds multiple files into one.")
            .arg(Arg::with_name("files")
                .short('f')
                .long("files")
                .value_name("FILES")
                .help("Specifies the list of files to meld.")
                .takes_value(true)
                .multiple(true)
                .required(true))
            .arg(Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Specifies the name of the output file.")
                .takes_value(true)
                .default_value("axe_output")
                .validator(|v| {
                    if v.trim().is_empty() {
                        Err(String::from("The file path must not be empty."))
                    } else {
                        Ok(())
                    }
                })));

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("CUT", cut_matches)) => {
            let file = cut_matches.value_of("file").unwrap();
            let count: u8 = cut_matches.value_of("count").unwrap_or("2").parse().expect("Count must be a number");
            if !Path::new(file).exists(){
                eprint!("Error: The file[{}] doesn't exist.\n", file);
                process::exit(1);
            } else if !Path::new(file).is_file() {
                eprint!("Error: The path[{}] is not a file.\n", file);
                process::exit(1);
            }
            join(file, count);
        }
        Some(("MELD", meld_matches)) => {
            let files: Vec<&str> = meld_matches.values_of("files").unwrap().collect();
            let output = meld_matches.value_of("output").unwrap_or("output.txt");
            if files.len() < 2 {
                eprint!("Error: You must send at least to files to meld.\n");
                process::exit(1);
            }
            for file in files.iter() {
                if !Path::new(file).exists(){
                    eprint!("Error: The file[{}] doesn't exist.\n", file);
                    process::exit(1);
                } else if !Path::new(file).is_file() {
                    eprint!("Error: The path[{}] is not a file.\n", file);
                    process::exit(1);
                }
            }
            meld(files, output);
        }
        _ => {
            app.print_help().unwrap();
            println!();
        }
    }
}
