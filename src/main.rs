use clap::{Arg, App, SubCommand};

#[cfg(not(debug_assertions))]
use config::VERSION;

#[cfg(debug_assertions)]
pub const VERSION: &str = "v0.0.0-debug";

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

fn write_file_bytes(path: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file: File = File::create(path)?;
    file.write_all(bytes)?;
    Ok(())
}

fn read_file_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_segment(file_name: &str, bytes: &[u8], index: usize, width: usize) {
    let output_file_name = format!("{}.{:0width$}.axe", file_name, index, width = width);
    match write_file_bytes(&output_file_name, &bytes) {
        Ok(_) => println!("Section: {} ", output_file_name),
        Err(e) => {
            eprintln!("Error: Cannot create the file: {}", e);
            process::exit(1);
        }
    }
}

fn cut_segments(file: &str, segments: Vec<usize>) {
    let bytes: Vec<u8>;
    match read_file_bytes(file) {
        Ok(bs) => bytes = bs,
        Err(e) => {
            eprintln!("Error: Cannot read the file: {}", e);
            process::exit(1);
        }
    }
    let segments_total: usize = segments.iter().sum();
    if segments_total > bytes.len() {
        eprintln!("Error: The size of the file is less than the expected segments: {} < {}", bytes.len(), segments_total);
        process::exit(1);
    }
    let file_name: &str = Path::new(file).file_name().unwrap().to_str().unwrap();
    let mut start: usize = 0;
    let mut end: usize = bytes.len();
    let width: usize = (segments.len() + 1).to_string().len();
    for (i, value) in segments.iter().enumerate() {
        end = *value + start;
        write_segment(file_name, &bytes[start..end], i + 1, width);
        start = end;
    }
    if segments_total < bytes.len() {
        end = bytes.len();
        write_segment(file_name, &bytes[start..end], segments.len() + 1, width);
    }
}

fn cut_count(file: &str, count: usize) {
    let bytes: Vec<u8>;
    match read_file_bytes(file) {
        Ok(bs) => bytes = bs,
        Err(e) => {
            eprintln!("Error: Cannot read the file: {}", e);
            process::exit(1);
        }
    }
    let segment_size: usize = bytes.len() / count;
    let file_name: &str = Path::new(file).file_name().unwrap().to_str().unwrap();
    let mut start: usize = 0;
    let mut end: usize = segment_size;
    let width: usize = (count).to_string().len();
    for i in 0..(count - 1) {
        write_segment(file_name, &bytes[start..end], i + 1, width);
        start = end;
        end = start + segment_size;
    }
    end = bytes.len();
    write_segment(file_name, &bytes[start..end], count, width);
}

fn meld(files: Vec<&str>, output: &str) {
    let mut meld_content: Vec<u8> = Vec::new();
    for file in files.iter() {
        let mut bytes: Vec<u8>;
        match read_file_bytes(file) {
            Ok(bs) => bytes = bs,
            Err(e) => {
                eprintln!("Error: Cannot read the file: {}", e);
                process::exit(1);
            }
        }
        meld_content.append(&mut bytes);
    }
    match write_file_bytes(&output, &meld_content) {
        Ok(_) => println!("Melded file: {} ", output),
        Err(e) => {
            eprintln!("Error: Cannot create the file: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    let mut app: App = App::new("Axe")
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
            .group(clap::ArgGroup::with_name("cut_options")
                .args(&["count", "segments"]))
            .arg(Arg::with_name("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Specifies how many files to create from the file.")
                .takes_value(true)
                .default_value("2")
                .validator(|v| {
                    v.parse::<usize>()
                        .map_err(|_| String::from("The count must be a valid integer."))
                        .and_then(|val| {
                            if val < 2 {
                                Err(String::from("The count must be at least 2."))
                            } else {
                                Ok(())
                            }
                        })
                }))
            .arg(Arg::with_name("segments")
                .short('s')
                .long("segments")
                .value_name("SEGMENTS")
                .help("Specifies the segments to split the file into, as a comma-separated list of byte positions.")
                .takes_value(true)
                .multiple(true)
                .use_delimiter(true)
                .validator(|v| {
                    v.split(',')
                        .map(|s| s.trim().parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| String::from("All segments must be valid integers."))
                        .and_then(|segments| {
                            if segments.is_empty() {
                                Err(String::from("At least one segment must be specified."))
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
            let file: &str = cut_matches.value_of("file").unwrap();
            let count: usize = cut_matches.value_of("count").unwrap_or("2").parse().expect("Count must be a number");
            let segments: Vec<usize> = cut_matches.values_of("segments")
                .map(|values| values.filter_map(|v| v.parse::<usize>().ok()).collect())
                .unwrap_or_else(Vec::new);
            if !Path::new(file).exists(){
                eprintln!("Error: The file[{}] doesn't exist.", file);
                process::exit(1);
            } else if !Path::new(file).is_file() {
                eprintln!("Error: The path[{}] is not a file.", file);
                process::exit(1);
            }
            if segments.is_empty() {
                cut_count(file, count);
            } else {
                cut_segments(file, segments);
            }
        }
        Some(("MELD", meld_matches)) => {
            let files: Vec<&str> = meld_matches.values_of("files").unwrap().collect();
            let output: &str = meld_matches.value_of("output").unwrap_or("output.txt");
            if files.len() < 2 {
                eprintln!("Error: You must send at least two files to meld.");
                process::exit(1);
            }
            for file in files.iter() {
                if !Path::new(file).exists(){
                    eprintln!("Error: The file[{}] doesn't exist.", file);
                    process::exit(1);
                } else if !Path::new(file).is_file() {
                    eprintln!("Error: The path[{}] is not a file.", file);
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
