# lcfetch

A command-line tool to fetch LeetCode problems and solve them locally. I made this because I enjoy my code editor, and dislike the one on LeetCode's UI. It's ass.

## Usage

```bash
lcfetch <number> <location> <language>
```

- `<number>` - LeetCode problem number
- `<location>` - Where to save the file
- `<language>` - Programming language (`py`, `js`, `ts`, `cpp`, `c`, `rs`)

## Example

```bash
lcfetch 23 ./leetcode_dir py
```

This fetches LeetCode problem #23, sets up a Python file in `./leetcode_dir`, and includes the test cases so you can run them locally.

## Supported Languages

- Python (`py`)
- Java ('java')
- C++ (`cpp`)
