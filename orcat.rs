// Copyright © 2021 Jakub Wilk <jwilk@jwilk.net>
// SPDX-License-Identifier: MIT

const PROG: &str = "orcat";

const BUFFER_SIZE: usize = 1 << 20; // 1 MiB

use std::cmp::max;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{Read, Write};
use std::io;
use std::process::exit;

fn usage()
{
    println!("Usage: {} FILE [FILE...]", PROG);
    exit(0);
}

fn usage_error()
{
    eprintln!("Usage: {} FILE [FILE...]", PROG);
    exit(1);
}

fn io_error(path: &OsStr, error: &io::Error)
{
    let path = path.to_string_lossy();
    eprintln!("{}: {}: {}", PROG, path, error);
    exit(1);
}

fn stdio_error(error: &io::Error)
{
    eprintln!("{}: {}", PROG, error);
    exit(1);
}

enum File {
    Stdin(io::Stdin),
    Fs(fs::File),
    None,
}

impl File {

    fn is_none(&self) -> bool
    {
        return match *self {
            File::None => true,
            _ => false,
        }
    }

    fn open(path: &OsStr) -> io::Result<File> {
        Ok({
            if path == "-" {
                File::Stdin(io::stdin())
            } else {
                File::Fs(fs::File::open(path)?)
            }
        })
    }

    fn read_all(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut asize: usize = 0;
        while asize < buf.len() {
            let size: usize = self.read(&mut buf[asize..])?;
            if size == 0 {
                break;
            }
            asize += size;
        }
        Ok(asize)
    }

}

impl io::Read for File {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            File::Stdin(ref mut file) => file.read(buf),
            File::Fs(ref mut file) => file.read(buf),
            File::None => unreachable!(),
        }
    }

}

fn main()
{
    let args: Vec<OsString> = env::args_os().collect();
    if args.len() < 2 {
        usage_error()
    }
    let arg1 = args[1].to_string_lossy();
    if arg1.starts_with('-') {
        match &arg1[..] {
            "-" => (),
            "-h" => usage(),
            "--help" => usage(),
            _ => usage_error(),
        };
    }
    let mut files: Vec<File> = vec![];
    for path in &args[1..] {
        let file: File = File::open(path)
            .unwrap_or_else(|exc| { io_error(path, &exc); unreachable!() });
        files.push(file);
    }
    let mut stdout = io::stdout();
    loop {
        let mut abuffer = [0u8; BUFFER_SIZE];
        let mut asize: usize = 0;
        for (cfile, path) in files.iter_mut().zip(&args[1..]) {
            if cfile.is_none() {
                continue;
            }
            let mut cbuffer = [0u8; BUFFER_SIZE];
            let csize: usize = cfile.read_all(&mut cbuffer[..])
                .unwrap_or_else(|exc| { io_error(path, &exc); unreachable!() });
            for i in 0..csize {
                abuffer[i] |= cbuffer[i];
            }
            asize = max(asize, csize);
            if csize < BUFFER_SIZE {
                *cfile = File::None;
            }
        }
        if asize == 0 {
            break;
        }
        stdout.write_all(&abuffer[..asize])
            .unwrap_or_else(|exc| { stdio_error(&exc); unreachable!() });
    }
}

// vim:ts=4 sts=4 sw=4 et
