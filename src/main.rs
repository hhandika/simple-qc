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

use std::time::Instant;

fn main() {
    let version = "0.1.5"; 
    
    let start_time = Instant::now();

    cli::process_fastq_commands(version);

    let duration = start_time.elapsed();

    println!("Execution time: {:?}", &duration);

}

