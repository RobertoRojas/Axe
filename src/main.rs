use clap::{Arg, App, SubCommand};

use config::VERSION;

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
                .required(true))
            .arg(Arg::with_name("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("Specifies how many files to create from the file.")
                .takes_value(true)
                .default_value("2")))
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
                .takes_value(true)));

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("CUT", cut_matches)) => {
            let file = cut_matches.value_of("file").unwrap();
            let count: usize = cut_matches.value_of("count").unwrap_or("2").parse().expect("Count must be a number");

            println!("Cutting file: {}", file);
            println!("Number of files to create: {}", count);
        }
        Some(("MELD", meld_matches)) => {
            let files: Vec<&str> = meld_matches.values_of("files").unwrap().collect();
            let output = meld_matches.value_of("output").unwrap_or("output.txt");

            println!("Melding files: {:?}", files);
            println!("Output file: {}", output);
        }
        _ => {
            app.print_help().unwrap();
            println!();
        }
    }
}
