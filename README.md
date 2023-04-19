# [External] BOM Folding Tool

This repository contains a libary + tool for converting BOM flat files into a format compatible with LightSource's ItemSync.

## Setup

There are three ways to get the tool:

1. From source with the [rust toolchain](https://rustup.rs/)
2. With docker using the included [Dockerfile](./Dockerfile)
3. Prebuilt executible included in the repo

## Running the tool

Running the cli with the `--help` flag will display all of the available options and their format.

| Input    | Description                                                                                                                                                                       |
| -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| --input  | Path to input CSV file. The CSV file must contain a the keys "Part Number", "Part Name", "Quantity" and "level" (capitalization is necessary).                                    |
| --output | (Optional) Output directory to write the two generated CSV files (BOMs and BOM Entries). If not set, then the tree-like BOM structure will be printend in debug format to stdout. |

NOTE: Future iterations of the tool will have greater flexibility on the names of the input headers.
