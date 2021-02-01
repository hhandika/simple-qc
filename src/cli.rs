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
                                .help("Process a single file")
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
                let mut gunzip = true;

                if fastq_matches.is_present("nogz") {
                    gunzip = true
                }
                process_dir(entry, gunzip, iscsv);

            } else if fastq_matches.is_present("file") {
                let entries: Vec<&str> = fastq_matches
                                        .values_of("file")
                                        .unwrap()
                                        .collect();
                process_multiple_entries(&entries, iscsv);

            } else if fastq_matches.is_present("wildcard") {
                let entries: Vec<&str> = fastq_matches.values_of("wildcard")
                                                    .unwrap()
                                                    .collect();
                process_multiple_entries(&entries, iscsv);
                
                
            } else if fastq_matches.is_present("wdir") {
                let val = fastq_matches.value_of("wdir").unwrap();
                io::traverse_dir(&val, iscsv);
                
            } else {
                println!("No command provided!");
            }
        }
        
        _ => unreachable!("Unreachable commands!"),
    };
}

#[inline(always)]
fn process_dir(entry: &str, gunzip: bool, iscsv: bool) {
    let input = PathBuf::from(&entry);
    let mut files = "*.fastq.gz";
    if !gunzip {
        files = "*.fastq";
    }
    
    let path = input.join(files);
    io::glob_dir(&path, iscsv);
}

#[inline(always)]
fn process_multiple_entries(entries: &[&str], iscsv: bool) {
    let files: Vec<PathBuf> = entries.iter()
                                .map(PathBuf::from)
                                .collect();
    let path = false;
    io::par_process_fastq_gz(&files, path, iscsv);
}