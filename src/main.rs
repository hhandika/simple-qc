//! Heru Handika
//! 28 December 2020
//! Read dir for qc
//! Lisence MIT

mod io;
mod fastq;
mod sequence;
mod qscores;
mod stats;
mod cli;

use std::time::Instant;

fn main() {
    let version = "0.2.0"; 
    
    let start_time = Instant::now();

    cli::process_fastq_commands(version);

    let duration = start_time.elapsed();

    println!("Execution time: {:?}", &duration);
    println!("\nThank you for using simpleQC v{}! 😊", &version);
}

