use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

use flate2::bufread::MultiGzDecoder;

use crate::sequence::{Fasta, SeqReads};

pub fn process_fasta(input: &PathBuf) -> Fasta {
    if is_gz_fasta(input) {
        parse_gz_fasta(input) 
    } else if is_unzip_fasta(input) {
        parse_unzip_fasta(input) 
        // parse_unzip_fasta(input)
    } else {
        panic!("INVALID FASTA");
    }
}

#[inline(always)]
fn is_gz_fasta(input: &PathBuf) -> bool {
    input.extension().unwrap() == "gz"
}

#[inline(always)]
fn is_unzip_fasta(input: &PathBuf) -> bool {
    let ext = input.extension().unwrap();

    ext == "fasta" || ext == "fas" || ext == "fs"
}

fn parse_gz_fasta(input: &PathBuf) -> Fasta {
    let file = File::open(input).unwrap();
    let read = BufReader::new(file);
    let decom = MultiGzDecoder::new(read);
    let buff = BufReader::new(decom);

    parse_fasta(buff, input)
}

fn parse_unzip_fasta(input: &PathBuf) -> Fasta {
    let file = File::open(input).unwrap();
    let buff = BufReader::new(file);
    
    parse_fasta(buff, input)
}

fn parse_fasta<R: BufRead>(buff: R, input: &PathBuf) -> Fasta {
    let stdout = io::stdout();
    let mut stdbuf = io::BufWriter::new(stdout);

    write!(stdbuf, "Processing {:?}\t",
        input.file_name().unwrap()).unwrap();

    let mut contig_counts: u32 = 0;
    let mut contigs: Vec<SeqReads> = Vec::new();
    buff.lines()
        .filter_map(|ok| ok.ok())
        .filter(|recs| !recs.is_empty())
        .for_each(|recs| {
            if recs.starts_with('>') {
                    contig_counts += 1;
            } else {
                let reads = SeqReads::get_seq_stats(&recs.trim().as_bytes());
                contigs.push(reads);
            }
                
        });
    // let mut reads = buff.lines();

    // // loop {
    // //     let seq = reads.by_ref()
    // //         .filter_map(|ok| ok.ok())
    // //         .filter(|recs| !recs.is_empty())
    // //         .take_while(|recs| !recs.starts_with(">"))
    // //         .collect::<String>();
    // //     let contig = SeqReads::get_seq_stats(&seq.as_bytes());
    // //     contigs.push(contig);
    // //     contig_counts += 1;
    // // }
    // while let Some(Ok(line)) = reads.by_ref().next() {
    //     if line.starts_with(">") {
    //         continue; }
    //     // } else {
    //     //     if is_fasta {
    //     let seq = reads.by_ref()
    //         .filter_map(|ok| ok.ok())
    //         .filter(|recs| !recs.is_empty())
    //         .take_while(|recs| !recs.starts_with(">"))
    //         .collect::<String>();
    //     let contig = SeqReads::get_seq_stats(&seq.as_bytes());
    //     contigs.push(contig);
    //     contig_counts += 1;
    // }
    
        
    writeln!(stdbuf, "DONE!").unwrap();
    
    Fasta::new(input, &contig_counts, &contigs)

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn process_fasta_test() {
        let input = PathBuf::from("test_files/contigs.fasta.gz");
        let in_unzip = PathBuf::from("test_files/contigs.fasta");

        let res = process_fasta(&input);
        let res_unzip = process_fasta(&in_unzip);
        
        assert_eq!(3, res.contig_counts);
        assert_eq!(3, res_unzip.contig_counts);
    }

    #[test]
    fn process_spaced_fasta_test() {
        let input = PathBuf::from("test_files/contigs_spaced.fasta");

        let res = process_fasta(&input);
        
        assert_eq!(3, res.contig_counts);
    }

    #[test]
    fn is_gz_fasta_test() {
        let fname = PathBuf::from("valid.fasta.gz");
        assert_eq!(true, is_gz_fasta(&fname));
    }

    #[test]
    fn is_unzip_fasta_test() {
        let fname = PathBuf::from("valid.fasta");
        let fname_fs = PathBuf::from("valid.fs");
        let fname_fas = PathBuf::from("valid.fas");

        assert_eq!(true, is_unzip_fasta(&fname));
        assert_eq!(true, is_unzip_fasta(&fname_fs));
        assert_eq!(true, is_unzip_fasta(&fname_fas));
    }
    
    #[test]
    #[should_panic]
    fn process_fasta_panic_test() {
        let fname = PathBuf::from("invalid.fastq");

        process_fasta(&fname);
    }

    #[test]
    #[should_panic]
    fn parse_fasta_test() {
        let input = PathBuf::from("test_files/invalid.fasta");

        process_fasta(&input);
    }
}