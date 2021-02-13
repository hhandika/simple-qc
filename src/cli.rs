//! Heru Handika
//! 
//! Command line parser

use std::path::PathBuf;

use clap::{App, AppSettings, Arg};

use crate::input;

pub fn get_cli(version: &str) {
    let args = App::new("simpleQC")
                .version(version)
                .about("A simple CLI app for NGS quality control.")
                .author("Heru Handika <hhandi1@lsu.edu>")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("fastq")
                        .about("Uses for FASTQ (raw-sequences) inputs")
                        .arg(
                            Arg::with_name("dir")
                                .short("d")
                                .long("dir")
                                .help("Inputs a single directory")
                                .conflicts_with_all(&["file", "wildcard", "wdir"])
                                .takes_value(true)
                                .value_name("DIR")
                            )

                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Inputs a single file")
                                .conflicts_with_all(&[ "dir", "wildcard", "wdir"])
                                .multiple(true)
                                .value_name("FASTQ FILE")
                            )

                        .arg(
                            Arg::with_name("wildcard")
                                .short("c")
                                .long("wildcard")
                                .help("Finds files using wildcards. Allows multiple inputs")
                                .conflicts_with_all(&[ "dir", "file","wdir"])
                                .multiple(true)
                                .value_name("WILDCARD")
                            )

                        .arg(
                            Arg::with_name("wdir")
                                .short("w")
                                .long("wdir")
                                .help("Tranverses through nested directories")
                                .conflicts_with_all(&[ "dir", "file", "wildcard"])
                                .takes_value(true)
                                .value_name("PARENT DIR")
                            )
                        
                        .arg(
                            Arg::with_name("nocsv")
                                .long("nocsv")
                                .help("Does not save results")
                                .takes_value(false)
                            )
                        
                        .arg(
                            Arg::with_name("nogz")
                                .long("nogz")
                                .help("Input is an uncompressed FASTQ files")
                                .conflicts_with_all(&["file", "wdir", "wildcard"])
                                .takes_value(false)
                            )
                    )
                .subcommand(
                    App::new("fasta")
                        .about("Uses for FASTA (sequence assemblies) inputs")
                        .arg(
                            Arg::with_name("dir")
                                .short("d")
                                .long("dir")
                                .help("Inputs a single directory")
                                .conflicts_with_all(&["file", "wildcard", "wdir"])
                                .takes_value(true)
                                .value_name("DIR")
                            )
                        
                        .arg(
                            Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Inputs FASTA files. Allows multiple inputs")
                                .conflicts_with_all(&[ "dir", "wildcard", "wdir"])
                                .multiple(true)
                                .value_name("FASTQ FILES")
                            )

                        .arg(
                            Arg::with_name("wildcard")
                                .short("c")
                                .long("wildcard")
                                .help("Finds files using wildcards. Allows multiple inputs")
                                .conflicts_with_all(&[ "dir", "file","wdir"])
                                .multiple(true)
                                .value_name("WILDCARDS")
                            )
                        
                        .arg(
                            Arg::with_name("wdir")
                                .short("w")
                                .long("wdir")
                                .help("Tranverses through nested directories")
                                .conflicts_with_all(&[ "dir", "file", "wildcard"])
                                .takes_value(true)
                                .value_name("PARENT DIR")
                            )
                        
                        .arg(
                            Arg::with_name("nocsv")
                                .long("nocsv")
                                .help("Does not save results")
                                .takes_value(false)
                            )
                        
                        .arg(
                            Arg::with_name("gz")
                                .long("gz")
                                .help("Input is a compressed FASTA file")
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
 
fn process_dir(entry: &str, query: &str, iscsv: bool, fastq: bool) {
    let input = PathBuf::from(&entry);
    let glob = format!("*.{}", query);
    let path = input.join(glob);
    input::glob_dir(&path, iscsv, fastq);
}

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