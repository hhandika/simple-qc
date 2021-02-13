use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Lines, Write};
use std::path::PathBuf;

use flate2::bufread::MultiGzDecoder;

use crate::sequence::{FastaStats, SeqReads};

pub fn process_fasta(input: &PathBuf) -> FastaStats {
    let file = File::open(input).unwrap();
    if is_gz_fasta(input) {
        let read = BufReader::new(file);
        let decom = MultiGzDecoder::new(read);
        parse_fasta(decom, input) 
    } else if is_unzip_fasta(input) {
        parse_fasta(file, input) 
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



fn parse_fasta<R: Read>(file: R, input: &PathBuf) -> FastaStats {
    let stdout = io::stdout();
    let mut stdbuf = io::BufWriter::new(stdout);

    write!(stdbuf, "Processing {:?}\t",
        input.file_name().unwrap()).unwrap();

    let mut contig_counts: u32 = 0;
    let mut contigs: Vec<SeqReads> = Vec::new();

    let file = Fasta::new(file);

    file.into_iter()
        .for_each(|recs| {
            contig_counts += 1;
            let reads = SeqReads::get_seq_stats(&recs.as_bytes());
            contigs.push(reads);
        });
        
    writeln!(stdbuf, "\x1b[0;32mDONE!\x1b[0m").unwrap();
    
    FastaStats::new(input, &contig_counts, &contigs)
}

pub struct Fasta<R> {
    reader: Lines<BufReader<R>>,
    pub id: bool,
    pub seq: String,
}

impl<R: Read> Fasta<R> {
    fn new(file: R) -> Self {
        Self {
            reader: BufReader::new(file).lines(),
            id: false,
            seq: String::new()
        }
    }
    
}

impl<R: Read> Iterator for Fasta<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Ok(line)) = self.reader.next() {
            if line.starts_with('>') {
                if self.id {                    
                    let mut res = String::new();
                    res.push_str(&self.seq);
                    self.id = true;
                    self.seq.clear();
                    return Some(res);
                } else {
                    self.id = true;
                    self.seq.clear();
                }
                continue;
            }
            self.seq.push_str(line.trim());
        }
        if self.id {
            let mut res = String::new();
            res.push_str(&self.seq);
            self.id = false;
            self.seq.clear();
            self.seq.shrink_to_fit();
            return Some(res);
        }
        None
    }
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
}