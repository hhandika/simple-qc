//! Heru Handika
//! 28 December 2020
//! Read dir for qc
mod parser;

use std::path::Path;
use std::time::Instant;

use glob::glob;
use rayon::prelude::*;
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

    let files: Vec<_> = glob(&path.to_string_lossy())
        .expect("Failed to read files")
        .filter_map(|recs| recs.ok()) 
        .collect();
    
    if files.is_empty() {
        panic!("Can't find fastq files.")
    }

    // println!("FOR LOOP PARSER"); // REMOVE AFTER TESTING
    // let timeit = Instant::now();
    // files.par_iter()
    //     .for_each(|entry| {
    //         parser::decompress_fastq(&entry);
    //     });
    // let duration = timeit.elapsed();
    
    // println!("CLOSURE PARSER"); // REMOVE AFTER TESTING
    let timeit = Instant::now();
    files.par_iter()
        .for_each(|recs| {
            parser::parse_fastq_gz(&recs);
        });
    let duration = timeit.elapsed();

    println!("Calculation time (Wall): {:?}", &duration);
}
