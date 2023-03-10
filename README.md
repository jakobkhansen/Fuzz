# Fuzz

Fuzz is a fuzzy-finder (à la [FZF](https://github.com/junegunn/fzf)) written in Rust.

![fuzz](https://user-images.githubusercontent.com/8071566/224407706-0ad3f7da-f20c-4513-aaaf-8b1fd55b7c6c.gif)

## Installation

```bash
git clone git@github.com:jakobkhansen/Fuzz.git
cd Fuzz
cargo install --path .
```

The `fuzz` command is now installed and ready to use.

## Usage

Fuzz can take any input via standard input and outputs the selected item to the standard
output. Combine commands with pipes or write scripts to create your own pickers!

Example usage:

```bash
fd . | fuzz | xargs vim # Open a file or directory in Vim (or any other editor)

cd $(fd . --type d | fuzz) # cd to a directory

git branch | fuzz | xargs git checkout # List git branches and checkout selected

git status --porcelain | fuzz | awk '{print $2}' | xargs git add # List modified git files and add selected
```

## Built-ins

Fuzz doesn't currently offer any built in pickers, but will offer built in pickers and a
Vim plugin in the future.

## Matching algorithm

Fuzz uses the Smith-Waterman algorithm to compute a local alignment score for each item.
Since Fuzz is currently single-threaded it can become quite slow when there are many items
and a long pattern to match.

The following values are used for the Smith-Waterman algorithm:

```rust
const MATCH: i32 = 16;
const MISMATCH: i32 = i32::MIN;
const GAP: i32 = -1;
```

Multipliers for special positions will be added, similarly to FZF.
