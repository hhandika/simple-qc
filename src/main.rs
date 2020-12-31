//! Heru Handika
//! 28 December 2020
//! Read dir for qc
mod io;
pub mod parser;

use std::path::Path;

use clap::{App, Arg};
// use indicatif::{HumanDuration};

fn main() {
    let args = App::new("RealFastQC-Exp")
        .version("0.1.0-alpha3.5")
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

    io::process_inputs(&path);
    
}
