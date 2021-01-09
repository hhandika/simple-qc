use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use flate2::bufread::MultiGzDecoder;

use crate::sequence::*;
use crate::qscores::*;

// Main driver for parsing compressed fastq files.
pub fn parse_fastq_gz(input: &PathBuf) -> Summary {
    check_input_file(&input);

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
                            panic!("{:?} IS INVALID FASTQ. \
                                LOOKING FOR '@' FOUND '{}' at line {}",
                                input, &recs, &idx + 1);
                        } else { reads += 1 }},

                1 => sq_per_read.push(SeqReads::count_reads(&recs.trim().as_bytes())),
                
                2 => { if !&recs.starts_with('+') {
                            panic!("{:?} IS INVALID FASTQ. \
                                LOOKING FOR '+' FOUND '{}' at line {}",
                                input, &recs, &idx + 1);
                        }},

                3 => qscores.push(QScore::analyze_qscores(&recs.trim().as_bytes())),

                _ => panic!("INVALID FASTQ!"),
            });

    let all_reads = Summary::count_all_reads(
                        input, &reads, &sq_per_read, &qscores);
    writeln!(outbuff, "\x1b[0;32mDONE!\x1b[0m").unwrap();
    all_reads
}

fn is_gunzipped_fastq(input: &PathBuf) -> bool {
    if input.extension().unwrap() != "gz" {
        false
    } else { 
        true 
    }
}

fn check_input_file(input: &PathBuf) {
    if !is_gunzipped_fastq(&input) {
        panic!("FILE INPUT IS NOT COMPRESSED FASTQ. \
                THE FILE EXTENSION SHOULD BE '.gz'")
    }
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
    #[should_panic(expected = "\"test_files/invalid.fastq.gz\" IS INVALID FASTQ. \
                                LOOKING FOR '+' FOUND '-' at line 3")]
    fn test_parsing_invplus_panic() {
        let input = PathBuf::from("test_files/invalid.fastq.gz");
        parse_fastq_gz(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid2.fastq.gz\" IS INVALID FASTQ. \
                                LOOKING FOR '@' FOUND 'Bunomys_chrysocomus' at line 9")]
    fn test_parsing_invname_panic() {
        let input = PathBuf::from("test_files/invalid2.fastq.gz");
        parse_fastq_gz(&input);
    }
    
    #[test]
    fn parsing_whitespaced_fastq_gz_test() {
        let input = PathBuf::from("test_files/whitespace.fastq.gz");
        let res = parse_fastq_gz(&input);

        assert_eq!(70, res.total_base);
        assert_eq!(0, res.sum_low_bases);
        assert_eq!(32.0, res.mean_qscores);
    }

    #[test]
    fn parsing_valid_fastq_qz_test() {
        let input = PathBuf::from("test_files/valid.fastq.gz");
        let res = parse_fastq_gz(&input);

        assert_eq!(140, res.total_base);
        assert_eq!(0, res.sum_low_bases);
        assert_eq!(0.0, res.n_content);
        assert_eq!(64, res.total_gc);
        assert_eq!(32.0, res.mean_qscores);
        assert_eq!(70, res.min_reads);
    }

    #[test]
    fn check_gunzip_input_test() {
        let input = PathBuf::from("valid_input.fastq.gz");

        assert_eq!(true, is_gunzipped_fastq(&input));
    }

    #[test]
    #[should_panic]
    fn panic_gunzip_input_test() {
        let input = PathBuf::from("valid_input.fastq");
        check_input_file(&input);
    }
}
