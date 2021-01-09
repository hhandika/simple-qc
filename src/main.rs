//! Heru Handika
//! 28 December 2020
//! Read dir for qc
//! Lisence MIT

mod io;
mod parser;
mod sequence;
mod qscores;
mod stats;
mod cli;

// use cli::*;
// use structopt::StructOpt;

// use std::path::Path;

// use clap::{App, Arg, AppSettings, SubCommand};
// use indicatif::{HumanDuration};
use std::time::Instant;

fn main() {
    let version = "0.1.4";
    
    let timeit = Instant::now();
    println!("Starting simpleQC v{}...", &version);
    
    cli::process_fastq_commands(version);

    let duration = timeit.elapsed();

    println!("Execution time: {:?}", &duration);

}

