use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use flate2::bufread::MultiGzDecoder;

use crate::sequence::*;
use crate::qscores::*;

pub fn process_fastq(input: &PathBuf) -> Fastq {
    if is_gunzip(input) {
        parse_gunzip_fastq(input)
    } else if is_unzip_fastq(input) {
        parse_unzip_fastq(input)
    } else {
        panic!("INVALID FASTQ.");
    }
}

#[inline(always)]
fn is_gunzip(input: &PathBuf) -> bool {
    input.extension().unwrap() == "gz"
}

fn is_unzip_fastq(input: &PathBuf) -> bool {
    let ext = input.extension().unwrap();

    ext == "fastq" || ext == "fq"
}

fn parse_gunzip_fastq(input: &PathBuf) -> Fastq {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let decompressor = MultiGzDecoder::new(reader);
    let buff = BufReader::new(decompressor);

    parse_fastq(buff, input)
}

fn parse_unzip_fastq(input: &PathBuf) -> Fastq {
    let file = File::open(input).unwrap();
    let buff = BufReader::new(file);
    
    parse_fastq(buff, input)
}

fn parse_fastq<R: BufRead>(buff: R, input: &PathBuf) -> Fastq {
    let stdout = io::stdout();
    let mut outbuff = io::BufWriter::new(stdout);

    write!(outbuff, "Processing {:?}\t", 
        &input.file_name().unwrap());

    let mut reads: u32 = 0;
    let mut sq_per_read: Vec<SeqReads> = Vec::new();
    let mut qscores: Vec<QScore> = Vec::new();

    buff.lines()
        .filter_map(|ok| ok.ok())
        .filter(|recs| !recs.is_empty())
        .enumerate()
        .for_each(|(idx, recs)| {
            match idx % 4 {
                0 => { 
                    if !&recs.starts_with('@') {
                    panic!("{:?} IS INVALID FASTQ. \
                        LOOKING FOR '@' FOUND '{}' at line {}",
                        input, &recs, &idx + 1);
                    } else { 
                        reads += 1 }
                    },

                1 => {
                    let reads = SeqReads::get_seq_stats(&recs.trim().as_bytes());
                    sq_per_read.push(reads);
                }
                
                2 => { 
                    if !&recs.starts_with('+') {
                        panic!("{:?} IS INVALID FASTQ. \
                            LOOKING FOR '+' FOUND '{}' at line {}",
                            input, &recs, &idx + 1);
                }},

                3 => qscores.push(QScore::analyze_qscores(&recs.trim().as_bytes())),

                _ => panic!("INVALID FASTQ!"),
        }});

    let all_reads = Fastq::count_all_reads(
        input, &reads, &sq_per_read, &qscores);
        
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
        parse_gunzip_fastq(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid.fastq.gz\" IS INVALID FASTQ. \
                                LOOKING FOR '+' FOUND '-' at line 3")]
    fn test_parsing_invplus_panic() {
        let input = PathBuf::from("test_files/invalid.fastq.gz");
        parse_gunzip_fastq(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid2.fastq.gz\" IS INVALID FASTQ. \
                                LOOKING FOR '@' FOUND 'Bunomys_chrysocomus' at line 9")]
    fn test_parsing_invname_panic() {
        let input = PathBuf::from("test_files/invalid2.fastq.gz");
        parse_gunzip_fastq(&input);
    }
    
    #[test]
    fn parsing_whitespaced_fastq_gz_test() {
        let input = PathBuf::from("test_files/whitespace.fastq.gz");
        let res = parse_gunzip_fastq(&input);

        assert_eq!(70, res.total_base);
        assert_eq!(0, res.sum_low_bases);
        assert_eq!(32.0, res.mean_qscores);
    }

    #[test]
    fn parsing_valid_fastq_qz_test() {
        let input = PathBuf::from("test_files/valid.fastq.gz");
        let res = parse_gunzip_fastq(&input);

        assert_eq!(140, res.total_base);
        assert_eq!(0, res.sum_low_bases);
        assert_eq!(0.0, res.n_content);
        assert_eq!(64, res.total_gc);
        assert_eq!(32.0, res.mean_qscores);
        assert_eq!(70, res.min_reads);
    }

    #[test]
    #[should_panic]
    fn panic_invalid_fastq_test() {
        let input = PathBuf::from("valid_input.fasta");
        process_fastq(&input);
    }

    #[test]
    fn is_gunzip_fastq_test() {
        let input = PathBuf::from("valid_input.fastq.gz");
        assert_eq!(true, is_gunzip(&input));
    }

    #[test]
    fn is_unzip_fastq_test() {
        let fastq_gz = PathBuf::from("valid_input.fastq.gz");
        let fastq = PathBuf::from("valid_input.fastq");
        let fq = PathBuf::from("valid_input.fq");

        assert_eq!(false, is_unzip_fastq(&fastq_gz));
        assert_eq!(true, is_unzip_fastq(&fastq));
        assert_eq!(true, is_unzip_fastq(&fq));
    }

    #[test]
    fn is_unzip_fastq_panic_test() {
        let invalid = PathBuf::from("invalid_input.fasta");
        assert_eq!(false, is_unzip_fastq(&invalid));
    }
}
