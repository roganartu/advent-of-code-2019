# advent-of-code-2019
Advent of Code 2019

Thought I'd try my hand at Rust. I've never written Rust before, don't expect it to be any good.

## Each day

To create a new day:
```sh
$ make day=1
```

### Add example and input

Example (for the small example with a known solution) and input files are already created
at `dayN/input/{example.txt,input.txt}`. Manually populate them from the AoC web page.

### Running and Testing

To run the day's code against the small example output:
```sh
$ make test day=1
```

To run the day's code against the input:
```sh
$ make run day=1
```
