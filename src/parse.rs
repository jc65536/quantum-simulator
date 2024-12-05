use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

use crate::prog::{Ins, Prog};

fn capture<T: FromStr>(re: &Regex, s: &str) -> T {
    // println!("{s}");
    re.captures(s.trim()).unwrap()[1]
        .parse()
        .ok()
        .expect("capture error")
}

fn split_head(s: &str) -> (&str, &str) {
    let mut split = s.split_whitespace();
    (split.next().expect("head 1"), split.next().expect("head 2"))
}

pub fn parse_file(file: File) -> Prog {
    let re_qreg: Regex = Regex::new(r"q\[(\d+)]").unwrap();
    let re_creg: Regex = Regex::new(r"c\[(\d+)]").unwrap();

    let reader = BufReader::new(file);

    let mut lines = reader
        .split(b';')
        .map(|res| String::from_utf8(res.unwrap()).unwrap().trim().to_string())
        .filter(|line| !line.is_empty());

    lines
        .next()
        .filter(|line| line == "OPENQASM 2.0")
        .expect("openqasm error");

    lines
        .next()
        .filter(|line| line == "include \"qelib1.inc\"")
        .expect("include error");

    let qreg: u32 = match split_head(&lines.next().unwrap()) {
        ("qreg", s) => capture(&re_qreg, s),
        _ => panic!("qreg"),
    };

    let creg: u32 = match split_head(&lines.next().unwrap()) {
        ("creg", s) => capture(&re_creg, s),
        _ => panic!("creg"),
    };

    let instrs = lines
        .map(|line| match split_head(&line) {
            ("h", s) => Ins::H(capture(&re_qreg, s)),
            ("x", s) => Ins::X(capture(&re_qreg, s)),
            ("t", s) => Ins::T(capture(&re_qreg, s)),
            ("tdg", s) => Ins::Tdg(capture(&re_qreg, s)),
            ("cx", s) => {
                let mut split = s.split(",");
                Ins::Cx(
                    capture(&re_qreg, split.next().expect("split")),
                    capture(&re_qreg, split.next().expect("split")),
                )
            }
            _ => panic!("invalid instr"),
        })
        .collect();

    Prog { qreg, creg, instrs }
}
