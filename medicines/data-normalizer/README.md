# Data Normalizer

## What is it for?

The data normalizer is a simple Perl script which accepts SQL server output TSV files and normalizes them into our input format.

Currently supported:

- SPC & PIL files.

To do:

- PAR files.

## How do I use it?

The script uses the following syntax:

`perl normalize.pl TYPE INPUT OUTPUT`

It takes three input variables:

- TYPE is the type of file to be normalized;
- INPUT is the TSV source file;
- OUTPUT is the CSV file to write to.

Example:

`perl normalize.pl SPCPIL downloads/input.tsv docs/spcpil/metadata.csv`
