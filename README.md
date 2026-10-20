# wordfreq

`wordfreq` is a small command-line tool. It counts the words in a text file
or in standard input, and prints the most frequent words first.

A word is a run of alphanumeric characters. The tool lowercases all words
before it counts them.

## Usage

```
wordfreq top <n> [file|-]
```

- `<n>` is the number of words to print.
- `[file]` is the input file. Use `-` or no file to read standard input.
- Each output line has the form `<count> <word>`.
- Words with the same count sort by name, A to Z.

If you give no count, the tool prints the top 10 words.

## Errors

Bad usage, a missing file, or a read error gives one message on
standard error and an exit code of 1.

## Examples

```
$ printf 'b a a\nb\n' | wordfreq top 5
2 a
2 b

$ wordfreq top 20 report.txt
```

## Build

You need Rust 1.70 or later.

```
cargo build --release
cargo test
```

The release binary is in `target/release/wordfreq`.

## License

MIT. See the `LICENSE` file.
