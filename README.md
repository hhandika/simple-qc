# simple-qc
![simple-qc](https://github.com/hhandika/simple-qc/workflows/simple-qc/badge.svg)

Simple (and fast) cli app to aid with quality control for high-throughput sequqencing data. SimpleQC provide sequence quality information with similar performance to using a combination of GNU Parallel, Awk/Shell scripts for line counting. The app can check a single file, multiple files in a folder, or tranverse nested directories to find compressed fastq files. Current working samples work with fastq. Fasta support and a wrapper to enhance FastQC are coming soon. 

# State of The Code
The code is fully working as intended, but it is still at the early stage. Please, expect constant re-structuring and refactoring. Currently, it only supports compressed fastq files. Fasta support is coming and potentially will add wrapper for fastqc to simplify QC workflow.  


