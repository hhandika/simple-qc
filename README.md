# simple-qc

![simple-qc](https://github.com/hhandika/simple-qc/workflows/simple-qc/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/simple-qc.svg?branch=main)](https://www.travis-ci.com/hhandika/simple-qc)


simpleQC is a high-performance, single executable command-line app to aid with quality control for high-throughput sequencing data. It is easy to setup and requires no-dependency to run. It supports sequence quality check for Illumina Fastq raw-reads and assemlies in Fasta format, whether the file is compressed (gunzip) or not. The app can check a single file, multiple files in a folder, or tranverse nested directories. The idea is that you will be able to get data quality information of your entire NGS files in a single command. The final result will be saved in a csv file.

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
  - [Quick Install](#quick-install)
  - [Compiling from sources](#compiling-from-sources)
- [Usages](#usages)
  - [Command Structure](#command-structure)
  - [Sample Usages](#sample-usages)
- [State of the Code](#state-of-the-code)
- [Acknowledgment](#acknowledgment)

## Quick Start

To install simpleQC, you can download the latest version of the app for your operating system [here](https://github.com/hhandika/simple-qc/releases). The installation is similar to any single executable command-line application: unzip the file and copy it to your path variable. On MacOS, you may receive unidentified developer errors. Allow the program to run in the [security settings](https://support.apple.com/en-us/HT202491). In Linux or MacOS, you may also need to add executable permission for the app using this command in your terminal: `chmod +x sqc`. If you are new to command line application, you could follow the installation instruction below.

### Using simpleQC

For raw-reads, you only need to change your directory to the parent directory of your files and then the command is as simple as below:

```Bash
sqc fastq -w .
```

It will scan every FASTQ files inside the folder and other folders nested within it. 

Similar command is also available for sequence assembly files in Fasta format.

```Bash
sqc fasta -w .
```

simpleQC can process a single directory, multiple wildcards, or multiple files. For more options, use help commands:

```Bash
sqc --help
```

For Fastq options:

```Bash
sqc fastq --help
```

For fasta options:

```Bash
sqc fasta --help
```

## Installation

### Quick Install

You can download the latest version of the app for your operating system [here](https://github.com/hhandika/simple-qc/releases). Then, follow the installation instruction for your specific operating system.

#### MacOS/Linux

MacOS usually will unzip the file automatically after the file has been downloaded. You will have a single executable called `sqc` in your Downloads folder. If the file still in zip, unzip the file first before continuing the installation process. 

In your terminal, change your directory to the folder where you store the program and then:

```Bash
./sqc --version
```

It should display the program version in the terminal. If it throws an error, it is more likely that the program does not have executable permission. Try:

```Bash
chmod +x sqc
```

However, this is not the most efficient way to  use the program. We will need to set it up, so that the program can be recognized by your terminal from anywhere in your system. You can move the executable file to the path that is already recognized by your terminal to look for executable, such as `/usr/local/bin/`.

```Bash
mv Downloads/sqc /usr/local/bin
```

If you already have a path register to your environment variable, you could just move the program the path. You can also create a new path and register it to your terminal. I tend to prefer to create one folder for a single executable app.  It can slow down your terminal startup if you have too many path registered in your environment variable. Other bioinformatic apps, such IQ-Tree, BPP, etch., are installed in similar fashion as simpleQC. I will include them all in the folder. If you don't have such folder yet, follow the instruction below:

First, we will create a directory to store a single executable app in your HOME directory.

```Bash
cd ~
```

Then, create a new folder in the directory. You could call it `programs` or anything you prefer.

```Bash
mkdir programs
```

Check which shell do you use. Most recent Mac will use zsh, and the older ones may still use bash. To check your shell:

```Bash
echo $SHELL
```

If it says zsh. Register the path below to .zshrc. If it is bash register it to .bash_profile.

If you don't care in which line the path should be added in your environment variable. You can use the command below. Change .zshrc to .bash_profile if your Mac still use bash.

```Bash
echo 'export PATH=$PATH:~/programs/' >> .zshrc
```

If you do care which line in the file the text should be appended, you can open the environment variable file using nano or vim:

```Bash
nano .zshrc
```

or

```Bash
nano .bash_profile
```

Then paste the text below:

```Bash
export PATH=$PATH:~/programs/
```

Next time you have a single executable app like simpleQC, you can just copy/move it to the `program` folder. Your terminal will recognize it right away. 

To check if sqc is properly installed. Try to check the program version again from anywhere in your system:

```Bash
sqc --version

```

If it displays the program version in your terminal, the program is ready to use.

### Compiling from sources

simpleQC requires rust compiler and C compiler. For most users, you will only need to install the Rust compiler toolchain available through [rust-lang website](https://www.rust-lang.org/tools/install).  

Compilation:

Clone the git repository from github.

```Bash
git clone https://github.com/hhandika/simple-qc.git
```

Change your directory to the program directory

```Bash
cd simple-qc/
```

Compile the program

```Bash
cargo build --release
```

Copy the executable to your path variable. 

```Bash
cp target/release/sqc [your-target-folder]
```

#### WSL users

You may receive linking errors during compilation because some WSL distros do not install C compiler by default. Hence, install C compiler first and try to re-compiled again. 

The easiest way is to install the C development package.

For Debian based distro (Debian, Ubuntu, etc...)

```Bash
sudo apt install build-essential
```

For openSUSE

```Bash
zypper se -t pattern devel
```

For Fedora

```Bash
sudo dnf groupinstall "Development Tools" "Development Libraries"
```

## Usages

### Command Structure

The command structure is as below.

```Bash
sqc <SUBCOMMAND> [OPTIONS] [FLAGS]
```

For example, if you want to process uncompressed fastq files.

```Bash
sqc fastq -d . --nogz
```

#### Subcommands

- `fastq` : to process raw-read sequences in fastq format.
- `fasta` : to process assembly sequences in fasta format.

#### Options

- `-d` or `--dir`     : for a single directory input.
- `-f` or `--file`    : for a file input. Support multiple file.
- `-w` or `--walk`    : to tranverse across nested directory.
- `-c` or `--wcard`   : process files using wild card.

#### Flags

Both `--nogz` and `--gz` flags only work with `--dir` options.

- `--nogz` : only available for fastq subcommand. It will search for non-compressed fastq instead.

- `--gz` : by default if you use fasta subcommand, it will search for uncompressed fasta files. Using This flag will instead search for compressed fasta files.

- `--nocsv`: does not save the result to csv. Display result on console only. 

- `--version` : check the program version number.

- `--help` : display help messages.

### Sample Usages

Tranverse nested directories for raw reads

```Bash
sqc fastq -w folder/
```

Process raw reads in a single directory

```Bash
sqc fastq -d folder/
```

## State of The Code

The code is fully working and well-tested. It is, however, still at the early stage. Please, expect constant re-structuring and refactoring. If you are just using the program, you should not need to worry about it. 

## Citations

I have no plan to publish it in a journal yet. My current plan is to eventually publish it in Zenodo. In the mean time, please use this Github repo if you cite this program.

## Acknowledgment

simpleQC is heavily inspired by [Phyluce](https://phyluce.readthedocs.io/en/latest/) pipeline. It was initially designed to tackle the limitation of the pipeline. Some of the ideas for the program was also inspired by [FastQC](https://www.bioinformatics.babraham.ac.uk/projects/fastqc/) program.  [Giovani Hernández-Canchola](https://scholar.google.com/citations?hl=en&user=B6rbNOEAAAAJ&view_op=list_works) tested the early version of the program and has provided invaluable feedback to further develop it. The iterator for Fasta was from [Rosetta Code](https://rosettacode.org/wiki/FASTA_format#Rust) that I modified to suit the needs for the program. Ultimately, thanks to [Rust-lang](https://www.rust-lang.org/) open-source developers and community in general for providing amazing documentations and libraries.

