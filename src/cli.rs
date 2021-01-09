//! Heru Handika
//! 
//! Command line parser

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};

use crate::io;

pub fn process_fastq_commands(version: &str) {
    let args = App::new("simpleQC")
                .version(version)
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
                                .multiple(true)
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
    
    println!("Starting simpleQC v{}...", &version);
    match args.subcommand() {
        ("fastq", Some(fastq_matches)) => {
            if fastq_matches.is_present("dir") {
                let val = fastq_matches.value_of("dir").unwrap();
                let input = PathBuf::from(&val);
                let files = "*.fastq.gz";
                let path = input.join(files);
                io::glob_dir(&path);

            } else if fastq_matches.is_present("file") {
                let val = fastq_matches.value_of("file").unwrap();
                let input = PathBuf::from(&val);
                io::process_file(&input);

            } else if fastq_matches.is_present("wildcard") {
                let val: Vec<&str> = fastq_matches.values_of("wildcard")
                                                .unwrap()
                                                .collect();
                let files: Vec<PathBuf> = val.iter()
                                            .map(PathBuf::from)
                                            .collect();
                io::par_process_dir(&files, false)
                
            } else if fastq_matches.is_present("wdir") {
                let val = fastq_matches.value_of("wdir").unwrap();
                io::traverse_dir(&val);
                
            } else {
                println!("No command provided!");
            }
        }
        
        _ => unreachable!("No commands!"),
    };
}