//! Heru Handika
//! 
//! Command line parser

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};

use crate::io;

pub fn process_fastq_commands(version: &str) {
    let args = App::new("simpleQC")
                .version("0.1.4")
                .about("Quickly count gc content from a fasta file")
                .author("Heru Handika <hhandi1@lsu.edu>")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                // Sub command for fastq
                .subcommand(
                    App::new("fastq")
                        .about("Process fastq files")
                        .arg(
                            Arg::with_name("dir")
                                .short("d")
                                .long("dir")
                                .help("Process fastq inside a directory")
                                .conflicts_with_all(&["file", "wildcard", "wdir"])
                                .takes_value(true)
                                .value_name("DIR")
                                // .multiple_values("true"),
                            )

                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Process a single file")
                                .conflicts_with_all(&[ "dir", "wildcard", "wdir"])
                                .takes_value(true)
                                .value_name("FASTQ FILE")
                            )

                        .arg(
                            Arg::with_name("wildcard")
                                .short("c")
                                .long("wildcard")
                                .help("Process a single file")
                                .conflicts_with_all(&[ "dir", "file","wdir"])
                                .takes_value(true)
                                .value_name("WILDCARD")
                            )

                        .arg(
                            Arg::with_name("wdir")
                                .short("w")
                                .long("wdir")
                                .help("Find all files inside dir and process it.")
                                .conflicts_with_all(&[ "dir", "file", "wildcard"])
                                .takes_value(true)
                                .value_name("PARENT DIR")
                            )
                    )
                .get_matches();

    match args.subcommand() {
        ("fastq", Some(fastq_matches)) => {
            if fastq_matches.is_present("dir") {
                let val = fastq_matches.value_of("dir").unwrap();
                let input = PathBuf::from(&val);
                let files = "*.fastq.gz";
                let path = input.join(files);

                println!("Initiating simpleQC v{}...", version);
                io::par_process_inputs(&path);

            } else if fastq_matches.is_present("file") {
                let val = fastq_matches.value_of("file").unwrap();
                println!("File name {}", &val);

            } else if fastq_matches.is_present("wildcard") {
                let val = fastq_matches.value_of("wildcard").unwrap();
                println!("File name {}", &val);
                
            } else if fastq_matches.is_present("wdir") {
                let val = fastq_matches.value_of("wdir").unwrap();
                println!("File name {}", &val);
                
            } else {
                println!("No command provided!");
            }
        }
        // ("fastq", Some(file_matches)) => {
        //         if file_matches.is_present("file") {
        //         let val = file_matches.value_of("file").unwrap();
        //         println!("File name {}", &val);
        //     }
        // }
        _ => unreachable!("No commands!"),
    };
}