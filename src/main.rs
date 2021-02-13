//! Heru Handika
//! First created: 28 December 2020
//! Lisence MIT

mod cli;
mod input;
mod fasta;
mod fastq;
mod sequence;
mod qscores;
mod stats;
mod output;

use std::time::Instant;

fn main() {
    let version = "0.3.2"; 
    
    let start_time = Instant::now();

    cli::get_cli(version);

    let duration = start_time.elapsed();

    println!("Execution time: {:?}", &duration);
    println!("\nThank you for using simpleQC v{}! 😊", &version);
}

