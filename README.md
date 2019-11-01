# Filesystem

## Installation

Clone the repo

```
git clone https://github.com/cgcardona/filesystem.git
```

```
cd filesystem
```

Build the app and deps

```
cargo build
```

Run the app

```
cargo run
   Compiling filesystem v0.1.0 (/Users/carloscardona/Sites/dev/rust-playground/filesystem)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/filesystem`
`mkdir a`
`touch a/b.txt`
`ls a`
Ok(
    ReadDir(
        "a",
    ),
)
`echo hello > a/b.txt`
`cat a/b.txt`
Ok(
    "hello world",
)
```
