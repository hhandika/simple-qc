//! Heru Handika
//! Module
//! 

use std::fs::File;
use std::io::{self, LineWriter, Write};
use std::path::PathBuf;
use std::sync::mpsc::channel;

use glob::glob;
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::parser;
use crate::sequence::Summary;

pub fn traverse_dir(path: &str, iscsv: bool) {
    let mut entries: Vec<PathBuf> = Vec::new();

    WalkDir::new(path).into_iter()
        .filter_map(|recs| recs.ok())
        .for_each(|e| {
            let files = String::from(e.path().to_string_lossy());
            match_fastq(&files, &mut entries);
        });
    
    let path = true;
    par_process_fastq_gz(&entries, path, iscsv);                            
}

fn match_fastq(files: &str, entries: &mut Vec<PathBuf>) {
    match files {
        s if s.ends_with(".fastq.gz") => entries.push(PathBuf::from(files)),
        s if s.ends_with(".fq.gz") => entries.push(PathBuf::from(files)),
        s if s.ends_with("fastq.gzip") => entries.push(PathBuf::from(files)),
        s if s.ends_with("fq.gzip") => entries.push(PathBuf::from(files)),
        _ => (),
    };
}

pub fn glob_dir(path: &PathBuf, iscsv: bool) {
    let files: Vec<PathBuf> = glob(&path.to_string_lossy())
        .expect("Failed to read files")
        .filter_map(|recs| recs.ok()) 
        .collect();
    
    if files.is_empty() {
        panic!("Can't find fastq files.");
    }

    par_process_fastq_gz(&files, false, iscsv);
}

// Process multiple files in parallel. 
pub fn par_process_fastq_gz(files: &[PathBuf], path: bool, iscsv: bool) {
    let (sender, receiver) = channel();
    
    files.into_par_iter()
        .for_each_with(sender, |s, recs| {
            s.send(parser::parse_fastq_gz(&recs)).unwrap();
        });
    
    let mut all_reads: Vec<Summary> = receiver.iter().collect();
    
    all_reads.sort_by(|a, b| a.seqname.cmp(&b.seqname));


    write_results(&all_reads, path, iscsv);


    println!("Total files: {}", all_reads.len());
}

fn write_results(results: &[Summary], path: bool, iscsv: bool) {
    println!("\n\x1b[1mResults:\x1b[0m");
    results.iter()
            .for_each(|recs| {
                    write_to_console(&recs);
                });
    if iscsv {
        write_to_csv(results, path);
    }
}

fn write_to_console(all_reads: &Summary) {
    let stdout = io::stdout();
    let mut buff = io::BufWriter::new(stdout);
    
    writeln!(buff, "\x1b[0;32mFile {:?}\x1b[0m", 
        &all_reads.seqname).unwrap();

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
    
    writeln!(buff, "Sequence length\t\t: {} bp\n", 
        &all_reads.total_base
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "\x1b[0;34mReads:\x1b[0m").unwrap();

    writeln!(buff, "Min\t\t\t: {} bp", 
        &all_reads.min_reads
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Max\t\t\t: {} bp", 
        &all_reads.max_reads
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Mean\t\t\t: {:.2} bp", 
        &all_reads.mean_reads).unwrap();
    
    writeln!(buff, "Median\t\t\t: {:.2} bp", 
        &all_reads.median_reads).unwrap();
    
    writeln!(buff, "Stdev\t\t\t: {:.2}\n", 
        &all_reads.sd_reads).unwrap();

    writeln!(buff, "\x1b[0;34mPhred Q-Scores:\x1b[0m").unwrap();

    writeln!(buff, "Mean\t\t\t: {:.2}",
    &all_reads.mean_qscores).unwrap();

    writeln!(buff, "Bases < 20\t\t: {}",
        &all_reads.sum_low_bases
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Low Q-score ratio\t: {:.2}\n",
        &all_reads.low_bases_ratio).unwrap();
    
    if all_reads.total_base != all_reads.sum_qlen {
        writeln!(buff, 
            "\x1b[0;33mWARNING!\n\
            \x1b[3mSome bases may not have Q-score.\n\
            The Q-score length and the sequence length are not equal.\
            \x1b[0m\n")
            .unwrap();
    }
    
}

fn write_to_csv(all_reads: &[Summary], path: bool) {
    let outname = "sQC-summary.csv";
    let output = File::create(outname).
                    expect("FILE EXISTS.");
    let mut line = LineWriter::new(output);

    write_csv_header(&mut line, path);
            
    all_reads.iter()
        .for_each(|seq| {
            write_csv_contents(seq, &mut line, path)
        });

    println!("Summary results is save as {}", outname);
}

fn write_csv_header<W: Write>(line:&mut W, path: bool) {
    if path {
        write!(line, "Path,").unwrap();
    }
    writeln!(line, 
        "Sequence names,\
        Read counts,\
        Total sequence length,\
        GC counts,\
        GC-content,\
        N counts,\
        N-content,\
        Min read length,\
        Max read length,\
        Mean read length,\
        Median read length,\
        Stdev read length,\
        Mean q-score,\
        Low base < 20,\
        Low q-score ratio")
        .unwrap();
}

fn write_csv_contents<W: Write>(seq: &Summary, line:&mut W, path: bool) {
    if path {
        write!(line, "{},", seq.path).unwrap();
    }
    writeln!(line, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", 
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
        seq.median_reads,
        seq.sd_reads,
        seq.mean_qscores,
        seq.sum_low_bases,
        seq.low_bases_ratio,
    ).unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn check_input_file_test() {
//         let input = "some_fastq.gz";
//         check_input_file(&input);
//     }   
// }