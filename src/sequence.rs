//! Heru Handika
//! 31 December 2020
//! Modules to process sequencing data
//! Lisence MIT

use std::path::{PathBuf};

pub struct SeqReads {
    pub seq_len: u32,
    pub gc_count: u32,
    pub n_count: u32,
}

impl SeqReads {
    pub fn count_reads(reads: &str) -> Self {
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
    pub fn count_all_reads(fname: &PathBuf, 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_count_test() {
        let a: String = String::from("AA");
        let b: String = String::from("AAGC");
        let c: String = String::from("aaAA");
        let d: String = String::from("aattggcc");
        let e: String = String::from("aataNctgn");

        let seq_a: SeqReads = SeqReads::count_reads(&a);
        let seq_b: SeqReads = SeqReads::count_reads(&b);
        let seq_c: SeqReads = SeqReads::count_reads(&c);
        let seq_d: SeqReads = SeqReads::count_reads(&d);
        let seq_e: SeqReads = SeqReads::count_reads(&e);

        assert_eq!(0, seq_a.gc_count);
        assert_eq!(2, seq_b.gc_count);
        assert_eq!(0, seq_c.gc_count);
        assert_eq!(4, seq_d.gc_count);
        assert_eq!(2, seq_a.seq_len);
        assert_eq!(4, seq_b.seq_len);
        assert_eq!(4, seq_c.seq_len);
        assert_eq!(8, seq_d.seq_len);
        assert_eq!(2, seq_e.n_count);
        assert_eq!(0, seq_a.n_count);
    }

    // #[test]
    // fn all_reads_test() {

    // }
    
}