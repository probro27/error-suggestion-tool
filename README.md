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
cargo run -- ls -la                       
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/code-errors ls -la`
Command: CheckedCommand { command: "ls" "-la" }
Result: Code: 0, error/output:
total 32
drwxr-xr-x@   9 prabhavkhera  staff   288 Dec 24 14:58 .
drwx------@ 161 prabhavkhera  staff  5152 Dec 24 02:37 ..
drwxr-xr-x@  13 prabhavkhera  staff   416 Dec 24 18:57 .git
-rw-r--r--@   1 prabhavkhera  staff     8 Dec 24 02:37 .gitignore
-rw-r--r--    1 prabhavkhera  staff   623 Dec 24 18:39 Cargo.lock
-rw-r--r--@   1 prabhavkhera  staff   206 Dec 24 18:39 Cargo.toml
-rw-r--r--    1 prabhavkhera  staff  1102 Dec 24 18:13 README.md
drwxr-xr-x@   4 prabhavkhera  staff   128 Dec 24 16:41 src
drwxr-xr-x@   5 prabhavkhera  staff   160 Dec 24 02:37 target
```

In case of errors a call to OPEN AI is made which gives a suggestion to the error provided:

```bash
cargo run -- cp
    Finished dev [unoptimized + debuginfo] target(s) in 2.81s
     Running `target/debug/code-errors cp`
Result: Code: 64, error/output:
usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpsvXx] source_file target_file
       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpsvXx] source_file ... target_directory
Suggestion is:
 
This error indicates that the user has entered an incorrect command while using the cp command on the linux command line. The user is likely trying to copy one or more files from one location to another. 

In order to resolve this error, the user should ensure that their command is in the proper form. The syntax for the cp command is provided in the error message, which reads as follows:

cp [-R [-H | -L | -P]] [-fi | -n] [-aclpsvXx] source_file target_file
cp [-R [-H | -L | -P]] [-fi | -n] [-aclpsvXx] source_file ... target_directory

For more information on the cp command, users can view the Linux manual page here: http://man7.org/linux/man-pages/man1/cp.1.html

Additionally, users can find tutorials and examples on the cp command here: https://www.geeksforgeeks.org/copy-command-linux-examples/
```

Right now, this is being made into a command line tool to download using `brew` or `apt-get`.
