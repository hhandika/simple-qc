//! Heru Handika
//! 
//! Command line parser

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};

use crate::input;

pub fn process_fastq_commands(version: &str) {
    let args = App::new("simpleQC")
                .version(version)
                .about("Quickly count gc content from a fasta file")
                .author("Heru Handika <hhandi1@lsu.edu>")
                .setting(AppSettings::SubcommandRequiredElseHelp)
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
                            )

                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Process a single file")
                                .conflicts_with_all(&[ "dir", "wildcard", "wdir"])
                                .multiple(true)
                                .value_name("FASTQ FILE")
                            )

                        .arg(
                            Arg::with_name("wildcard")
                                .short("c")
                                .long("wildcard")
                                .help("Finds files using wildcards")
                                .conflicts_with_all(&[ "dir", "file","wdir"])
                                .multiple(true)
                                .value_name("WILDCARD")
                            )

                        .arg(
                            Arg::with_name("wdir")
                                .short("w")
                                .long("wdir")
                                .help("Find all files inside dir and process it")
                                .conflicts_with_all(&[ "dir", "file", "wildcard"])
                                .takes_value(true)
                                .value_name("PARENT DIR")
                            )
                        
                        .arg(
                            Arg::with_name("nocsv")
                                .long("nocsv")
                                .help("Do not save results")
                                .takes_value(false)
                            )
                        
                        .arg(
                            Arg::with_name("nogz")
                                .long("nogz")
                                .help("Unzip fastq input")
                                .conflicts_with_all(&["file", "wdir", "wildcard"])
                                .takes_value(false)
                            )
                    )
                .subcommand(
                    App::new("fasta")
                        .about("Process fasta files")
                        .arg(
                            Arg::with_name("dir")
                                .short("d")
                                .long("dir")
                                .help("Process fasta inside a directory")
                                .conflicts_with_all(&["file", "wildcard", "wdir"])
                                .takes_value(true)
                                .value_name("DIR")
                            )
                        
                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Process a single file")
                                .conflicts_with_all(&[ "dir", "wildcard", "wdir"])
                                .multiple(true)
                                .value_name("FASTQ FILE")
                            )

                        .arg(
                            Arg::with_name("wildcard")
                                .short("c")
                                .long("wildcard")
                                .help("Finds files using wildcards")
                                .conflicts_with_all(&[ "dir", "file","wdir"])
                                .multiple(true)
                                .value_name("WILDCARD")
                            )
                        
                        .arg(
                            Arg::with_name("wdir")
                                .short("w")
                                .long("wdir")
                                .help("Find all files inside dir and process it")
                                .conflicts_with_all(&[ "dir", "file", "wildcard"])
                                .takes_value(true)
                                .value_name("PARENT DIR")
                            )
                        
                        .arg(
                            Arg::with_name("nocsv")
                                .long("nocsv")
                                .help("Do not save results")
                                .takes_value(false)
                            )
                        
                        .arg(
                            Arg::with_name("gz")
                                .long("gz")
                                .help("Gunzip fasta input")
                                .conflicts_with_all(&["file", "wdir", "wildcard"])
                                .takes_value(false)
                            )
                )
                .get_matches();
    
    println!("Starting simpleQC v{}...", &version);
    match args.subcommand() {

        ("fastq", Some(fastq_matches)) => {
            let mut iscsv = true;
            if fastq_matches.is_present("nocsv") {
                iscsv = false;
            }

            if fastq_matches.is_present("dir") {
                let entry: &str = fastq_matches.value_of("dir").unwrap();
                let mut ext = String::new();

                if fastq_matches.is_present("nogz") {
                    ext.push_str("fastq");
                } else {
                    ext.push_str("fastq.gz");
                }

                process_dir(entry, &ext, iscsv, true);

            } else if fastq_matches.is_present("file") {
                let entries: Vec<&str> = fastq_matches
                    .values_of("file").unwrap().collect();
                process_multiple_files(&entries, iscsv, true);

            } else if fastq_matches.is_present("wildcard") {
                let entries: Vec<&str> = fastq_matches
                    .values_of("wildcard").unwrap().collect();

                process_multiple_files(&entries, iscsv, true);
                
            } else if fastq_matches.is_present("wdir") {
                let entry = fastq_matches.value_of("wdir").unwrap();
                input::traverse_dir(&entry, iscsv, true); // true for fastq
                
            } else {
                println!("No command provided!");
            }
        }
        
        ("fasta", Some(fasta_matches)) => {
            let mut iscsv = true;
            if fasta_matches.is_present("nocsv") {
                iscsv = false;
            }

            if fasta_matches.is_present("dir") {
                let entry: &str = fasta_matches.value_of("dir").unwrap();
                let mut ext = String::new();

                if fasta_matches.is_present("gz") {
                    ext.push_str("fasta.gz");
                } else {
                    ext.push_str("fasta");
                }

                process_dir(entry, &ext, iscsv, false);

            } else if fasta_matches.is_present("file") {
                let entries: Vec<&str> = fasta_matches
                    .values_of("file").unwrap().collect();
                process_multiple_files(&entries, iscsv, false); // false for fasta

            } else if fasta_matches.is_present("wildcard") {
                let entries: Vec<&str> = fasta_matches
                    .values_of("wildcard").unwrap().collect();

                process_multiple_files(&entries, iscsv, false);
                
            } else if fasta_matches.is_present("wdir") {
                let entry: &str = fasta_matches.value_of("wdir").unwrap();
                input::traverse_dir(&entry, iscsv, false); // false for fasta

            } else {
                println!("No command provided!");
            }
        }


        _ => unreachable!("Unreachable commands!"),
    };
}

#[inline(always)]
fn process_dir(entry: &str, query: &str, iscsv: bool, fastq: bool) {
    let input = PathBuf::from(&entry);
    let glob = format!("*.{}", query);
    let path = input.join(glob);
    input::glob_dir(&path, iscsv, fastq);
}

#[inline(always)]
fn process_multiple_files(entries: &[&str], iscsv: bool, fastq: bool) {
    let files: Vec<PathBuf> = entries.iter()
        .map(PathBuf::from).collect();

    let path = false;

    if fastq {
        input::par_process_fastq(&files, path, iscsv);
    } else {
        input::par_process_fasta(&files, path, iscsv);
    }
    
}