# Error Suggestion Tool

Building a Command Line Tool which provides suggestions when developers face errors while coding!

## How it works

- Give a command to run `code-errors npm run dev`.
- It will run the command given and the error we get, should be parsed and sent to OPEN AI.
- Print the suggestion given by OPEN AI.

## Basic Usage Built

**Input**: `cargo run --ls -l -a`

**Result**:

```bash
cargo run -- ls -l -a
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/code-errors ls -l -a`
Command: "ls" "-l" "-a"
total 32
drwxr-xr-x@   9 user  staff   288 Dec 24 14:58 .
drwx------@ 161 user  staff  5152 Dec 24 02:37 ..
drwxr-xr-x@  13 user  staff   416 Dec 24 02:39 .git
-rw-r--r--@   1 user  staff     8 Dec 24 02:37 .gitignore
-rw-r--r--    1 user  staff   155 Dec 24 02:37 Cargo.lock
-rw-r--r--@   1 user  staff   180 Dec 24 02:37 Cargo.toml
-rw-r--r--    1 user  staff   325 Dec 24 16:26 README.md
drwxr-xr-x@   4 user  staff   128 Dec 24 16:41 src
drwxr-xr-x@   5 user  staff   160 Dec 24 02:37 target
Result: Code: 0, error: Ran successfully
```
