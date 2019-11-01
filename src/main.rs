use std::fs;
use std::fs::{File, OpenOptions, ReadDir};
use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

fn cat(path: &Path) -> io::Result<String> {
    let mut f: File = File::open(path)?;
    let mut s: String = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f: File = File::create(path)?;

    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn mkdir(path: &Path) -> io::Result<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn ls(path: &Path) -> io::Result<std::fs::ReadDir> {
    match fs::read_dir(path) {
        Err(e) => Err(e),
        Ok(paths) => Ok(paths),
    }
}

fn main() {
    println!("`mkdir a`");
    mkdir(&Path::new("a")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/b.txt`");
    touch(&Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ls a`");
    let r: Result<ReadDir, Error> = ls(&Path::new("a"));
    println!("{:#?}", r);

    println!("`echo hello > a/b.txt`");
    echo("hello world", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`cat a/b.txt`");
    let f: Result<String, Error> = cat(&Path::new("a/b.txt"));
    println!("{:#?}", f);
}
