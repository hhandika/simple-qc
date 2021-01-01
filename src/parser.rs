use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::{PathBuf};

use flate2::bufread::MultiGzDecoder;

// use crate::seqprocessor;
use crate::sequence::*;
use crate::qscores::*;

// Main driver for parsing compressed fastq files.
pub fn parse_fastq_gz(input: &PathBuf) -> Summary {
    let f = File::open(input).unwrap();
    let r = BufReader::new(f);
    let d = MultiGzDecoder::new(r);
    let buff = BufReader::new(d);

    let stdout = io::stdout();
    let mut outbuff = io::BufWriter::new(stdout);

    write!(outbuff, "Processing {}\t", 
        &input.file_name().unwrap().to_string_lossy()).unwrap();

    let mut reads: u32 = 0;
    let mut sq_per_read: Vec<SeqReads> = Vec::new();
    let mut qscores: Vec<QScore> = Vec::new();

    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(idx, recs)|
            match idx % 4 {
                0 => { if !&recs.starts_with('@') {
                            panic!("{:?} IS INVALID FASTQ. LOOKING FOR '@' FOUND '{}' at line {}",
                                &input, &recs, &idx + 1);
                        } else { reads += 1 }},
                1 => sq_per_read.push(SeqReads::count_reads(&recs)), 
                2 => { if !&recs.starts_with('+') {
                            panic!("{:?} IS INVALID FASTQ. LOOKING FOR '+' FOUND '{}' at line {}",
                                &input, &recs, &idx + 1);
                    }},
                3 => qscores.push(QScore::analyze_qscores(&recs.as_bytes())),
                _ => panic!("INVALID FASTQ!"),
            });

    let all_reads: Summary = Summary::count_all_reads(&input, &reads, &sq_per_read, &qscores);
    writeln!(outbuff, "\x1b[0;32mDONE!\x1b[0m").unwrap();
    all_reads
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_parsing_all_panic() {
        let input = PathBuf::from("test_files/invalid_fastq.fastq.gz");
        parse_fastq_gz(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid.fastq.gz\" IS INVALID FASTQ. LOOKING FOR '+' FOUND '-' at line 3")]
    fn test_parsing_invplus_panic() {
        let input = PathBuf::from("test_files/invalid.fastq.gz");
        parse_fastq_gz(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid2.fastq.gz\" IS INVALID FASTQ. LOOKING FOR '@' FOUND 'Bunomys_chrysocomus' at line 9")]
    fn test_parsing_invname_panic() {
        let input = PathBuf::from("test_files/invalid2.fastq.gz");
        parse_fastq_gz(&input);
    }
    
}
