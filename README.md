# simple-qc
![simple-qc](https://github.com/hhandika/simple-qc/workflows/simple-qc/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/simple-qc.svg?branch=main)](https://www.travis-ci.com/hhandika/simple-qc)


simpleQC is a high-performance, single executable command-line app to aid with quality control for high-throughput sequencing data. It is easy to setup and requires no-dependency to run. It supports sequence quality check for Illumina Fastq raw-reads and assemlies in Fasta format, whether the file is compressed (gunzip) or not. The app can check a single file, multiple files in a folder, or tranverse nested directories. The idea is that you will be able to get data quality information of your entire NGS files in a single command. The final result will be saved in a csv file.

## Quick Start
To install simpleQC, you can download the latest version of the app for your operating system [here](https://github.com/hhandika/simple-qc/releases). The installation is similar to any single executable command-line application: unzip the file and copy it to your path variable. On MacOS, you may receive unidentified developer errors. Allow the program to run in the [security settings](https://support.apple.com/en-us/HT202491). In Linux or MacOS, you may also need to add executable permission for the app using this command in your terminal: `chmod +x sqc`.

### Using simpleQC

For raw-reads, you only need to change your directory to the parent directory of your files and then the command is as simple as below:

```
sqc fastq -w .
```

It will scan every FASTQ files inside the folder and other folders nested within it. 

Similar command is also available for sequence assembly files in Fasta format.

```
sqc fasta -w .
```

simpleQC can process a single directory, multiple wildcards, or multiple files. For more options, use help commands:

```
sqc --help
```

For Fastq options:

```
sqc fastq --help
```

For fasta options:

```
sqc fasta --help
```

# Installation
## Compiling from sources
simpleQC requires rust compiler and C compiler. For most users, you will only need to install the Rust compiler toolchain available through [rust-lang website](https://www.rust-lang.org/tools/install).  

Compilation:

1. Clone the git repository from github.

```
git clone https://github.com/hhandika/simple-qc.git
```

2. Change your directory to the program directory

```
cd simple-qc/
```

3. Compile the program

```
cargo build --release
```

4. Copy the executable to your path variable. 

```
cp target/release/sqc [your-target-folder]
```

### For WSL users:

You may receive linking errors during compilation because some WSL distros do not install C compiler by default. Hence, install C compiler first and try to re-compiled again. 

The easiest way is to install the C development package. 

For Debian based distro (Debian, Ubuntu, etc...)
```
sudo apt install build-essential
```

For openSUSE
```
zypper se -t pattern devel
```

For Fedora
```
sudo dnf groupinstall "Development Tools" "Development Libraries"
```

## State of The Code
The code is fully working and well-tested. It is, however, still at the early stage. Please, expect constant re-structuring and refactoring. If you are just using the program, you should not need to worry about it. 

## Citations
I have no plan to publish it in a journal yet. My current plan is to eventually publish it in Zenodo. In the mean time, please use this Github repo if you cite this program.

## Acknowledgment
simpleQC is heavily inspired by [Phyluce](https://phyluce.readthedocs.io/en/latest/) pipeline. It was initially designed to tackle the limitation of the pipeline. Some of the ideas for the program was also inspired by [FastQC](https://www.bioinformatics.babraham.ac.uk/projects/fastqc/) program.  [Giovani Hernández-Canchola](https://scholar.google.com/citations?hl=en&user=B6rbNOEAAAAJ&view_op=list_works) tested the early version of the program and has provided invaluable feedback to further develop it. The iterator for Fasta was from [Rosetta Code](https://rosettacode.org/wiki/FASTA_format#Rust) that I modified to suit the needs for the program. Ultimately, thanks to [Rust-lang](https://www.rust-lang.org/) open-source developers and community in general for providing amazing documentations and libraries. 
