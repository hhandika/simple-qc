use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::{PathBuf};

use flate2::bufread::MultiGzDecoder;
use num_format::{Locale, ToFormattedString};

// pub fn decompress_fastq(input: &PathBuf) {
//     let f = File::open(input).unwrap();
//     let r = BufReader::new(f);
//     let mut d = MultiGzDecoder::new(r);

//     let mut v = Vec::new();
//     d.read_to_end(&mut v).unwrap();

//     let s = String::from_utf8_lossy(&v).into_owned();
    
//     let res = String::from(s);
//     let read = res.lines();
//     let mut reads: u32 = 0;
//     let mut seq_per_read: Vec<u32> = Vec::new();

//     for (i, recs) in read.enumerate() {
//         match i % 4 {
//             0 => {if !&recs.starts_with("@") {
//                     panic!("Invalid Sequences");
//                     } else {reads += 1}},
//             1 => {seq_per_read.push(count_reads(&recs.to_string()))}, 
//             2 => {if !&recs.starts_with("+") {
//                 panic!("Invalid Sequences");
//                 } else {continue}},
//             3 => continue,
//             _ => {println!("Invalid fastq"); break},
//         };
//     }

//     let seq_len: u32 = seq_per_read.iter().sum();
//     // let av: f64 = sum / read_size.len() as f64;
    
//     println!("\x1b[0;32mFile {:?}\x1b[0m", &input);
//     println!("No. of Reads: {}", 
//         &reads.to_formatted_string(&Locale::en));
//     println!("Sequence length: {} bp\n", 
//         &seq_len.to_formatted_string(&Locale::en));
// }

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

struct TotSeq {
    seqname: String,
    read_count: u32,
    total_base: u32,
    min_reads: u32,
    max_reads: u32,
    total_gc: u32,
    gc_content: f64,
    tot_n_count: u32,
    n_content: f64,
}

impl TotSeq {
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

fn write_results_to_console(all_reads: &TotSeq) {
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
        &all_reads.tot_n_count
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "N-content\t\t: {:.4}", 
        &all_reads.n_content).unwrap();

    writeln!(buff, "Min read length\t\t: {} bp", 
        &all_reads.min_reads
        .to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Max read length\t\t: {} bp", 
        &all_reads.max_reads
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Total sequence length\t: {} bp\n", 
        &all_reads.total_base
        .to_formatted_string(&Locale::en)).unwrap();
}


//////// STREAMING //////
pub fn parse_fastq_gz(input: &PathBuf) {
    let f = File::open(input).unwrap();
    let r = BufReader::new(f);
    let d = MultiGzDecoder::new(r);
    let buff = BufReader::new(d);

    // let stdout = io::stdout();
    // let mut outbuff = io::BufWriter::new(stdout);

    // write!(outbuff, "Processing {:?}\t", &input.file_name()).unwrap();

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

    let all_reads: TotSeq = TotSeq::count_all_reads(&input, &reads, &sq_per_read);
    // writeln!(outbuff, "DONE!");
    write_results_to_console(&all_reads);
}


/////// BETTER PARSER --CLEANER //////////////
// pub fn parse_fastq_gz(input: &PathBuf) {
//     let f = File::open(input).unwrap();
//     let r = BufReader::new(f);
//     let mut d = MultiGzDecoder::new(r);

//     let mut v = Vec::new();
//     d.read_to_end(&mut v).unwrap();
//     let s = String::from_utf8_lossy(&v).into_owned();
    
//     let seqs = String::from(s);
//     let mut reads: u32 = 0;
//     let mut seq_per_read: Vec<u32> = Vec::new();

//     seqs.lines()
//         .enumerate()
//         .for_each(|(idx, recs)|
//             match idx % 4 {
//                 0 => { if !&recs.starts_with("@") {
//                         panic!("INVALID FASTQ. LOOKING FOR @ FOUND {} at {}",
//                                 &recs, &idx + 1);
//                     } else {reads += 1}},
//                 1 => {seq_per_read.push(count_reads(&recs.to_string()))}, 
//                 2 => { if !&recs.starts_with("+") {
//                         panic!("INVALID FASTQ. LOOKING FOR + FOUND {} at {}",
//                                 &recs, &idx + 1);
//                     }},
//                 3 => (),
//                 _ => panic!("INVALID FASTQ!"),
//             });

//     let seq_len: u32 = seq_per_read.iter().sum();
    
//     let stdout = io::stdout();
//     let mut buff = io::BufWriter::new(stdout);

//     writeln!(buff, "\x1b[0;32mFile {:?}\x1b[0m", &input).unwrap();
//     writeln!(buff, "No. of Reads: {}", 
//         &reads.to_formatted_string(&Locale::en)).unwrap();
//     writeln!(buff, "Sequence length: {} bp\n", 
//         &seq_len.to_formatted_string(&Locale::en)).unwrap();
// }

//// END HERE ///

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
    
    // #[deny(soft_unstable)]
    // #[bench]
    // fn bench_fastq_parser_loop(b: &mut Bencher) {
    //     b.iter(|| {
    //         let input = PathBuf::from("data/gz_test_exome.fastq.gz");
    //         decompress_fastq(&input);
    //     })

    // }
}
