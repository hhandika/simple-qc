//! Heru Handika
//! Module
//! 
use std::time::Instant;
use std::fs::File;
use std::io::{self, LineWriter, Write};
use std::path::{PathBuf};
use std::sync::mpsc::channel;

use glob::glob;
use rayon::prelude::*;
use num_format::{Locale, ToFormattedString};

use crate::parser;
use crate::sequence::Summary;

pub fn process_inputs(path: &PathBuf) {
    let files: Vec<_> = glob(&path.to_string_lossy())
        .expect("Failed to read files")
        .filter_map(|recs| recs.ok()) 
        .collect();
    
    if files.is_empty() {
        panic!("Can't find fastq files.");
    }
    
    let (sender, receiver) = channel();

    let timeit = Instant::now();
    files.into_par_iter()
        .for_each_with(sender, |s, recs| {
            s.send(parser::parse_fastq_gz(&recs)).unwrap();
        });
    
    let all_reads: Vec<_> = receiver.iter().collect();
    
    println!("\n\x1b[1mResults:\x1b[0m");
    all_reads.iter()
            .for_each(|recs| {
                write_results_to_console(&recs);
            });

    write_to_csv(&all_reads);

    let duration = timeit.elapsed();
    println!("Total files: {}", all_reads.len());

    println!("Execution time: {:?}", &duration);
}

fn write_results_to_console(all_reads: &Summary) {
    let stdout = io::stdout();
    let mut buff = io::BufWriter::new(stdout);

    writeln!(buff, "\x1b[0;32mFile {:?}\x1b[0m", &all_reads.seqname).unwrap();
    writeln!(buff, "No. of reads\t\t: {}", 
        &all_reads.read_count
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Total GC count\t\t: {}", 
        &all_reads.total_gc
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "GC-content\t\t: {:.2}", 
        &all_reads.gc_content).unwrap();
    
    writeln!(buff, "Total N count\t\t: {}", 
        &all_reads.total_n
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "N-content\t\t: {:.4}", 
        &all_reads.n_content).unwrap();

    writeln!(buff, "Min read length\t\t: {} bp", 
        &all_reads.min_reads
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Max read length\t\t: {} bp", 
        &all_reads.max_reads
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Mean read length\t: {:.2} bp", 
        &all_reads.mean_reads).unwrap();
    
    writeln!(buff, "Median read length\t: {:.2} bp", 
        &all_reads.median_reads).unwrap();

    writeln!(buff, "Total sequence length\t: {} bp\n", 
        &all_reads.total_base
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "\x1b[0;34mPhred Q-Scores:\x1b[0m").unwrap();

    writeln!(buff, "Mean\t\t\t: {:.2}",
    &all_reads.mean_qscores).unwrap();

    writeln!(buff, "Bases < 20\t\t: {}",
        &all_reads.sum_low_bases
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Low Q-score ratio\t: {:.2}\n",
        &all_reads.low_bases_ratio).unwrap();
    
    if &all_reads.total_base != &all_reads.sum_qlen {
        writeln!(buff, "\x1b[0;33mWARNING!\n\
                        \x1b[3mSome bases may not have Q-score.\n\
                        Q-score and the sequence length are equal.\
                        \x1b[0m\n")
                        .unwrap();
    }
    
}

fn write_to_csv(all_reads: &[Summary]) {
    let outname = "sQC-summary.csv";
    let output = File::create(outname).
            expect("FILE EXISTS.");
    let mut line = LineWriter::new(output);

    writeln!(line, "Sequence names,\
                Read counts,\
                Total sequence length,\
                GC counts,\
                GC-content,\
                N counts,\
                N-content,\
                Min read length,\
                Max read length,\
                Mean read length,\
                Mean Q-Score length,\
                Mean Q-Score").unwrap();
        
    all_reads.iter()
        .for_each(|seq| {
            writeln!(line, "{},{},{},{},{},{},{},{},{},{},{},{},{}", 
                seq.seqname,
                seq.read_count,
                seq.total_base,
                seq.total_gc, 
                seq.gc_content,
                seq.total_n, 
                seq.n_content,
                seq.min_reads,
                seq.max_reads,
                seq.mean_reads,
                seq.mean_qscores,
                seq.sum_low_bases,
                seq.low_bases_ratio,
            ).unwrap();
        });

    println!("Summary results is save as {}", outname);
}