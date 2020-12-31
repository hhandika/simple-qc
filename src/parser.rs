use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::{PathBuf};

use flate2::bufread::MultiGzDecoder;

struct SeqReads {
    seq_len: u32,
    gc_count: u32,
    n_count: u32,
}

impl SeqReads {
    fn count_reads(reads: &str) -> Self {
        let mut sq = Self {
                    seq_len: reads.chars().count() as u32,
                    gc_count: 0,
                    n_count: 0,
                };

        reads.chars().for_each(|base|
                match base {
                    'G' | 'g'  => sq.gc_count += 1,
                    'C' | 'c' => sq.gc_count += 1,
                    'N' | 'n' => sq.n_count += 1,
                    _ => (), 
                });
        sq                   
    }
    
}

pub struct AllReads {
    pub seqname: String,
    pub read_count: u32,
    pub total_base: u32, 
    pub min_reads: u32,
    pub max_reads: u32,
    pub total_gc: u32,
    pub gc_content: f64,
    pub tot_n_count: u32,
    pub n_content: f64,
}

impl AllReads {
    fn count_all_reads(fname: &PathBuf, 
                    reads: &u32,
                    vec: &[SeqReads]) -> Self {
        let mut seq = Self {
            seqname: fname.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
            read_count: *reads,
            total_base: vec.iter().map(|v| v.seq_len).sum(),
            min_reads: vec.iter().map(|v| v.seq_len).min().unwrap(),
            max_reads: vec.iter().map(|v| v.seq_len).max().unwrap(),
            total_gc: vec.iter().map(|v| v.gc_count).sum(),
            gc_content: 0.0,
            tot_n_count: vec.iter().map(|v| v.n_count).sum(),
            n_content: 0.0,
        }; 
        seq.gc_content = seq.total_gc as f64 / seq.total_base as f64;
        seq.n_content = seq.tot_n_count as f64 / seq.total_base as f64;

        seq
    } 
}

// Main driver for parsing compressed fastq files.
pub fn parse_fastq_gz(input: &PathBuf) -> AllReads {
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

    buff.lines()
        .filter_map(|ok| ok.ok())
        .enumerate()
        .for_each(|(idx, recs)|
            match idx % 4 {
                0 => { if !&recs.starts_with('@') {
                                panic!("{:?} IS INVALID FASTQ. LOOKING FOR @ FOUND {} at line {}",
                                    &input, &recs, &idx + 1);
                        } else { reads += 1 }},
                1 => sq_per_read.push(SeqReads::count_reads(&recs)), 
                2 => { if !&recs.starts_with('+') {
                            panic!("{:?} IS INVALID FASTQ. LOOKING FOR + FOUND {} at line {}",
                                &input, &recs, &idx + 1);
                    }},
                3 => (),
                _ => panic!("INVALID FASTQ!"),
            });

    let all_reads: AllReads = AllReads::count_all_reads(&input, &reads, &sq_per_read);
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
    #[should_panic(expected = "\"test_files/invalid.fastq.gz\" IS INVALID FASTQ. LOOKING FOR + FOUND - at line 3")]
    fn test_parsing_invplus_panic() {
        let input = PathBuf::from("test_files/invalid.fastq.gz");
        parse_fastq_gz(&input);
    }

    #[test]
    #[should_panic(expected = "\"test_files/invalid2.fastq.gz\" IS INVALID FASTQ. LOOKING FOR @ FOUND Bunomys_chrysocomus at line 9")]
    fn test_parsing_invname_panic() {
        let input = PathBuf::from("test_files/invalid2.fastq.gz");
        parse_fastq_gz(&input);
    }

    #[test]
    fn gc_count() {
        let a: String = String::from("AA");
        let b: String = String::from("AAGC");
        let c: String = String::from("aaAA");
        let d: String = String::from("aattggcc");

        let seq_a: SeqReads = SeqReads::count_reads(&a);
        let seq_b: SeqReads = SeqReads::count_reads(&b);
        let seq_c: SeqReads = SeqReads::count_reads(&c);
        let seq_d: SeqReads = SeqReads::count_reads(&d);

        assert_eq!(0, seq_a.gc_count);
        assert_eq!(2, seq_b.gc_count);
        assert_eq!(0, seq_c.gc_count);
        assert_eq!(4, seq_d.gc_count);
    }
    
}
