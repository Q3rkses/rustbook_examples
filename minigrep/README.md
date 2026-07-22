# minigrep

A tiny, colorful `grep` clone in Rust to search a file or a whole directory for a word and get every matching line back with the match highlighted, plus its file and line number. An extended take on the classic [Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) I/O project, with colored output and directory search added on top.

## Features

- Highlighted matches (bold red), via [`colored`](https://crates.io/crates/colored)
- Search a single file or a whole directory (non-recursive)
- Case-insensitive mode with the `IGNORE_CASE` environment variable
- Every hit reports its file and line number

## Usage

```bash
cargo run -- <query> <path>                 # search a file or directory
cargo run -- <query>                        # path defaults to "."
IGNORE_CASE=1 cargo run -- <query> <path>   # case-insensitive
```

The `poems/` directory ships with two poems to try it out on:

- `im_nobody_who_are_you.txt`, _I'm Nobody! Who are you?_ by Emily Dickinson
- `no_man_is_an_island.txt`, _No Man Is an Island_ by John Donne

For example:

```bash
cargo run -- man poems/                              # every line with "man" across the folder
IGNORE_CASE=1 cargo run -- how poems/im_nobody_who_are_you.txt
```

## Demo

Searching a whole directory:

![directory search](screenshots/grep1.png)

Case-insensitive search:

![case-insensitive search](screenshots/grep2.png)

## How it works

| Piece                                | Job                                                              |
| ------------------------------------ | ---------------------------------------------------------------- |
| `Config::build`                      | Parses the CLI args and reads `IGNORE_CASE` from the environment |
| `search` / `search_case_insensitive` | Walk the lines, collect the hits, and paint the matched term red |
| `search_file`                        | Reads a single file and searches its contents                    |
| `search_directory`                   | Reads each regular file in a folder and searches them all        |

`main.rs` inspects the given path: if it is a directory it dispatches to `search_directory`, if it is a file it uses `search_file`, and otherwise it bails out with an error.

One quirk worth noting: in case-insensitive mode the matched line is lowercased before printing, so the output is case-normalized (for example `How dreary` prints as `how dreary`).
