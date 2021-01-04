//! Heru Handika
//! 28 December 2020
//! Read dir for qc
//! Lisence MIT

mod io;
mod parser;
mod sequence;
mod qscores;
mod stats;

use std::path::Path;

use clap::{App, Arg};
// use indicatif::{HumanDuration};

fn main() {
    let version = "0.1.4";
    let args = App::new("simpleQC")
        .version(version)
        .about("Quickly count gc content from a fasta file.")
        .arg(Arg::with_name("input")
            .help("Fastq file to analyze.")
            .takes_value(true)
            .required(true))
        .get_matches();


    let user_input = args.value_of("input").unwrap();
    let input = Path::new(user_input);
    let files = "*.fastq.gz";
    let path = input.join(files);

    println!("Initiating simpleQC v{}...", version);
    io::par_process_inputs(&path);
    
}
