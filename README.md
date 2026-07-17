# wordcount CLI

A simple command-line tool built in Rust to count word frequencies in a text file. It's designed to help learn Rust's core concepts like ownership, borrowing, `Result` handling, iterators, and manual argument parsing.
It's entirely built for learning purposes only.


## Features

*   Reads text from a specified file.
*   Counts the occurrences of each word.
*   Converts all words to lowercase and strips non-alphabetic characters for case-insensitive and clean counting.
*   Displays words sorted by their frequency in descending order.
*   Optionally shows only the top N most frequent words.

## Usage

```bash
./wordcount <filename> [--top N]
```

**Arguments:**

*   `<filename>`: The path to the text file you want to analyze. (Required)
*   `--top N`: (Optional) Displays only the top `N` most frequent words. `N` should be a positive integer.

## Examples

Count all word frequencies in `my_document.txt`:

```bash
./wordcount my_document.txt
```

Show the top 10 most frequent words in `my_document.txt`:

```bash
./wordcount my_document.txt --top 10
```
