//! Heru Handika
//! 31 December 2020
//! Modules to process sequencing data
//! Lisence MIT

use std::path::{PathBuf};

use crate::qscores::*;
use crate::stats::*;

pub struct SeqReads {
    pub seq_len: u32,
    pub gc_count: u32,
    pub n_count: u32,
}

impl SeqReads {
    pub fn count_reads(reads: &[u8]) -> Self {
        let mut sq = Self {
                    seq_len: reads.iter().count() as u32,
                    gc_count: 0,
                    n_count: 0,
                };

        reads.iter().for_each(|base|
                match base {
                    b'G' | b'g'  => sq.gc_count += 1,
                    b'C' | b'c' => sq.gc_count += 1,
                    b'N' | b'n' => sq.n_count += 1,
                    _ => (), 
                });
        sq                   
    }
    
}


// Add Q-len and low bases
pub struct Summary {
    pub path: String,
    pub seqname: String,
    pub read_count: u32,
    pub total_base: u32, 
    pub min_reads: u32,
    pub max_reads: u32,
    pub mean_reads: f64,
    pub median_reads: f64,
    pub sd_reads: f64,
    pub total_gc: u32,
    pub gc_content: f64,
    pub total_n: u32,
    pub n_content: f64,
    pub sum_qlen: u32,
    pub mean_qscores: f64,
    pub sum_low_bases: u32,
    pub low_bases_ratio: f64
}

impl Summary {
    pub fn count_all_reads(fname: &PathBuf, 
                            reads: &u32,
                            vec: &[SeqReads], 
                            qscores: &[QScore]
        ) -> Self {
        let seq_len = vec.iter().map(|v| v.seq_len).collect::<Vec<u32>>();

        let mut seq = Self {
            path: fname.parent().unwrap().to_string_lossy().into_owned(),
            seqname: fname.file_name().unwrap().to_string_lossy().into_owned(),
            read_count: *reads,
            total_base: seq_len.iter().sum(),
            min_reads: *seq_len.iter().min().unwrap(),
            max_reads: *seq_len.iter().max().unwrap(),
            mean_reads: 0.0,
            median_reads: median(&seq_len),
            sd_reads: 0.0,
            total_gc: vec.iter().map(|v| v.gc_count).sum(),
            gc_content: 0.0,
            total_n: vec.iter().map(|v| v.n_count).sum(),
            n_content: 0.0,
            sum_qlen: qscores.iter().map(|q| q.q_len).sum(),
            mean_qscores: 0.0,
            sum_low_bases: qscores.iter().map(|q| q.low_bases).sum(),
            low_bases_ratio: 0.0
        }; 

        seq.gc_content = seq.total_gc as f64 / seq.total_base as f64;
        seq.n_content = seq.total_n as f64 / seq.total_base as f64;
        seq.mean_reads = seq.total_base as f64 / seq.read_count as f64;
        seq.sd_reads = stdev(&seq_len, &seq.mean_reads);

        let sum_qscores: f64 = qscores.iter().map(|q| q.mean_q).sum();
        seq.mean_qscores = sum_qscores / seq.read_count as f64;
        seq.low_bases_ratio = seq.sum_low_bases as f64 / seq.total_base as f64;
        seq
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gc_count_test() {
        let a = String::from("AA");
        let b = String::from("AAGC");
        let c = String::from("aaAA");
        let d = String::from("aattggcc");
        let e = String::from("aataNctgn");
        let f = b"aacc";

        let seq_a: SeqReads = SeqReads::count_reads(&a.as_bytes());
        let seq_b: SeqReads = SeqReads::count_reads(&b.as_bytes());
        let seq_c: SeqReads = SeqReads::count_reads(&c.as_bytes());
        let seq_d: SeqReads = SeqReads::count_reads(&d.as_bytes());
        let seq_e: SeqReads = SeqReads::count_reads(&e.as_bytes());
        let seq_f: SeqReads = SeqReads::count_reads(f);

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
        assert_eq!(4, seq_f.seq_len);
        assert_eq!(2, seq_f.gc_count);
        assert_eq!(0, seq_f.n_count);
    }

    #[test]
    fn all_reads_test() {
        let a: String = String::from("ttggcc");
        let b: String = String::from("taNctgncca");
        
        let q = QScore {
                q_len: 2,
                mean_q: 40.0,
                low_bases: 0
            };
        let q_two = QScore {
                q_len: 2,
                mean_q: 40.0,
                low_bases: 0
            };
        
        let mut seq: Vec<SeqReads> = Vec::new();
        seq.push(SeqReads::count_reads(&a.as_bytes()));
        seq.push(SeqReads::count_reads(&b.as_bytes()));
        
        let qscores: Vec<QScore> = vec![q, q_two];

        let fname = PathBuf::from("data/test.fastq");
        let reads = 2;

        let res = Summary::count_all_reads(&fname, &reads, &seq, &qscores);

        assert_eq!("test.fastq", res.seqname);
        assert_eq!(2, res.read_count);
        assert_eq!(16, res.total_base);
        assert_eq!(8, res.total_gc);
        assert_eq!(0.5, res.gc_content);
        assert_eq!(2, res.total_n);
        assert_eq!(0.125, res.n_content);
        assert_eq!(6, res.min_reads);
        assert_eq!(10, res.max_reads);
        assert_eq!(8.0, res.mean_reads);
        assert_eq!(0, res.sum_low_bases);
        assert_eq!(40.0, res.mean_qscores);
        assert_eq!(0.0, res.low_bases_ratio);
    }
    
}