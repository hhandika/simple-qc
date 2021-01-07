//! Heru Handika
//! 6 January 2020
//! Module to parse command line
//! arguments.


use std::path::PathBuf;
use structopt::StructOpt;
// use std::path::Path;

use crate::io;

#[derive(StructOpt)]
#[structopt(
    name = "simpleQC",
    version = "0.1.4",
    about = "Quickly count gc content from a fasta file.",
    author = "Heru Handika <hhandi1@lsu.edu>"
)]
struct Cli {
    #[structopt(subcommand)]
    cmds: Option<Cmd>
}


#[derive(StructOpt)]
enum Cmd {
    /// Process fastq files
    #[structopt(name = "fastq")]
    Fastq (Opt)
}


#[derive(StructOpt)]
struct Opt {
    #[structopt(
            short= "d", 
            long="dir",
            help="Enter path",
            parse(from_os_str))
        ]
        dir: PathBuf,

    // #[structopt(
    //         short= "wc", 
    //         long="wildcard",
    //         help="Use wildcard",
    //         parse(from_os_str))
    //     ]
    //     wildcard: PathBuf
}


pub fn process_fastq_commands(version: &str) {
    let args = Cli::from_args();
    
    if let Some(subcommand) = args.cmds {
        match subcommand {
            Cmd::Fastq(opt) => {
                    let input = PathBuf::from(&opt.dir);
                    let files = "*.fastq.gz";
                    let path = input.join(files);

                    println!("Initiating simpleQC v{}...", version);
                    io::par_process_inputs(&path);
                }
            // Cmd::Fastq(opt) => {
            //     println!("Initiating simpleQC v{}...", version);
            //     io::par_process_inputs(&opt.wildcard);
            // }
            _ => unreachable!("Fail!"),
        };
    }
}