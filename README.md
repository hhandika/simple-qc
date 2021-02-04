# simple-qc
![simple-qc](https://github.com/hhandika/simple-qc/workflows/simple-qc/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/simple-qc.svg?branch=main)](https://www.travis-ci.com/hhandika/simple-qc)


simpleQC is a high-performance, zero dependency, cli app to aid with quality control for high-throughput sequencing data.  The app can check a single file, multiple files in a folder, or tranverse nested directories to find compressed fastq files. The idea is that you will be able to get data quality information of your entire NGS files in a single command. The final result will be saved in a csv file. 

# State of The Code
The code is fully working as intended, but it is still at the early stage. Please, expect constant re-structuring and refactoring. Currently, it only supports compressed fastq files. Fasta support is coming.  


