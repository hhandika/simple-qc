use std::fs::File;
use std::io::{self, LineWriter, Write};
use num_format::{Locale, ToFormattedString};

use crate::sequence::{FastqStats, FastaStats};

pub fn write_fastq(results: &mut [FastqStats], path: bool, iscsv: bool) {
    results.sort_by(|a, b| a.seqname.cmp(&b.seqname));

    println!("\n\x1b[1mResults:\x1b[0m");
    results.iter()
            .for_each(|recs| {
                    write_fastq_console(&recs);
                });
    
    println!("Total files: {}", results.len());

    if iscsv {
        write_fastq_csv(results, path);
    }
}

pub fn write_fasta(stats: &mut [FastaStats], path: bool, iscsv: bool) {
    stats.sort_by(|a, b| a.seqname.cmp(&b.seqname));

    println!("\n\x1b[1mResults:\x1b[0m");
    stats.iter()
        .for_each(|recs| {
            write_fasta_console(&recs);
        });
    
    println!("Total files: {}", stats.len());
    
    if iscsv {
        write_fasta_csv(stats, path);
    }
}

fn write_fasta_console(contigs: &FastaStats) {
    let stdout = io::stdout();
    let mut buff = io::BufWriter::new(stdout);

    writeln!(buff, "\x1b[0;32mFile {:?}\x1b[0m", contigs.seqname).unwrap();

    writeln!(buff, "No. of contigs\t\t: {}", 
        contigs.contig_counts.to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Total GC count\t\t: {}", 
        contigs.total_gc.to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "GC-content\t\t: {:.2}", 
        &contigs.gc_content).unwrap();

    writeln!(buff, "Total N count\t\t: {}", 
        &contigs.total_n
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "N-content\t\t: {:.4}", 
        &contigs.n_content).unwrap();

    writeln!(buff, "Sequence length\t\t: {} bp\n", 
        contigs.total_bp.to_formatted_string(&Locale::en)).unwrap();
    
    //---------------------------
    writeln!(buff, "\x1b[0;34mContigs:\x1b[0m").unwrap();
    writeln!(buff, "Min\t\t\t: {} bp", 
        contigs.min.to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Max\t\t\t: {} bp", 
        &contigs.max.to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Mean\t\t\t: {:.2} bp", 
        contigs.mean).unwrap();
    
    writeln!(buff, "Median\t\t\t: {:.2} bp", 
        &contigs.median).unwrap();

    writeln!(buff, "Stdev\t\t\t: {:.2}", 
        &contigs.sd).unwrap();
    
    writeln!(buff, "N50\t\t\t: {}", 
        &contigs.n50.to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "N75\t\t\t: {}", 
        &contigs.n75.to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "N90\t\t\t: {}", 
        &contigs.n90.to_formatted_string(&Locale::en)).unwrap();


    writeln!(buff, "Contigs >750 bp\t\t: {}", 
        &contigs.con750.to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Contigs >1000 bp\t: {}", 
        &contigs.con1000.to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff, "Contigs >1500 bp\t: {}", 
        &contigs.con1500.to_formatted_string(&Locale::en)).unwrap();
    
    writeln!(buff).unwrap();
}

fn write_fastq_console(all_reads: &FastqStats) {
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
        &all_reads.total_bp
        .to_formatted_string(&Locale::en)).unwrap();

    //---------------------------
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

    //--------------------
    writeln!(buff, "\x1b[0;34mPhred Q-Scores:\x1b[0m").unwrap();

    writeln!(buff, "Mean\t\t\t: {:.2}",
    &all_reads.mean_qscores).unwrap();

    writeln!(buff, "Bases < 20\t\t: {}",
        &all_reads.sum_low_bases
        .to_formatted_string(&Locale::en)).unwrap();

    writeln!(buff, "Low Q-score ratio\t: {:.2}\n",
        &all_reads.low_bases_ratio).unwrap();
    
    if all_reads.total_bp != all_reads.sum_qlen {
        writeln!(buff, 
            "\x1b[0;33mWARNING!\n\
            \x1b[3mSome bases may not have Q-score.\n\
            The Q-score length and the sequence length are not equal.\
            \x1b[0m\n")
            .unwrap();
    }
    
}

fn write_fastq_csv(all_reads: &[FastqStats], path: bool) {
    let fname = "sQC-Fastq.csv";
    let output = File::create(&fname).expect("FILE EXISTS.");
    let mut line = LineWriter::new(output);

    write_fastq_header(&mut line, path);
    
    all_reads.iter()
    .for_each(|seq| {
        write_fastq_contents(seq, &mut line, path)
    });
    
    println!("The result is saved as {}", fname);
}

fn write_fasta_csv(stats: &[FastaStats], path: bool) {
    let fname = "sQC-Fasta.csv";
    let output = File::create(&fname).expect("FILE EXISTS.");
    let mut line = LineWriter::new(output);

    write_fasta_header(&mut line, path);
    
    stats.iter()
    .for_each(|seq| {
        write_fasta_contents(seq, &mut line, path)
    });
    
    println!("The result is saved as {}", fname);
}

fn write_fastq_header<W: Write>(line:&mut W, path: bool) {
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
        Low q-score ratio"
    ).unwrap();
}

fn write_fastq_contents<W: Write>(seq: &FastqStats, line:&mut W, path: bool) {
    if path {
        write!(line, "{},", seq.path).unwrap();
    }
    writeln!(line, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", 
        seq.seqname,
        seq.read_count,
        seq.total_bp,
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

fn write_fasta_header<W: Write>(line:&mut W, path: bool) {
    if path {
        write!(line, "Path,").unwrap();
    }
    writeln!(line, 
        "Sequence_names,\
        Contig_counts,\
        Total_sequence_length,\
        GC_counts,\
        GC-content,\
        N_counts,\
        N-content,\
        Min_contig_length,\
        Max_contig_length,\
        Mean_contig_length,\
        Median_contig_length,\
        Stdev_contig_length,\
        N50,\
        N75,\
        N90,\
        No_contigs_>750bp,\
        No_contigs_>1000bp,\
        No_contigs_>1500bp"
    ).unwrap();
}

fn write_fasta_contents<W: Write>(seq: &FastaStats, line:&mut W, path: bool) {
    if path {
        write!(line, "{},", seq.path).unwrap();
    }
    writeln!(line, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", 
        seq.seqname,
        seq.contig_counts,
        seq.total_bp,
        seq.total_gc, 
        seq.gc_content,
        seq.total_n, 
        seq.n_content,
        seq.min,
        seq.max,
        seq.mean,
        seq.median,
        seq.sd,
        seq.n50,
        seq.n75,
        seq.n90,
        seq.con750,
        seq.con1000,
        seq.con1500,
    ).unwrap();
}