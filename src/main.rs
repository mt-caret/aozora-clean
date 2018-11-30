extern crate libc;
extern crate regex;

use regex::Regex;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let re = Regex::new(r"[［《〔（].*[］》〕）｜※]").unwrap();

    let mut bar_count = 0;
    let result = buffer
        .lines()
        .skip_while(|line| {
            if line.starts_with("----") {
                bar_count += 1;
                true
            } else {
                2 > bar_count
            }
        })
        .take_while(|line| !line.starts_with("底本"))
        .map(|line| re.replace(line, ""))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .concat()
        .replace("。", "。\n")
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", result);
    Ok(())
}
