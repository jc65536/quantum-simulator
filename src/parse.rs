use std::{collections::HashSet, io::{BufRead, BufReader, Read}};

use regex::Regex;

use crate::prog::{Ins, Prog};

fn capture(re: &Regex, s: &str) -> u32 {
    re.captures(s.trim()).unwrap()[1].parse().unwrap()
}

fn split_head(s: &str) -> (&str, &str) {
    let mut split = s.split_whitespace();
    (split.next().unwrap(), split.next().unwrap())
}

pub fn parse<R: Read>(input: R) -> Result<Prog, &'static str> {
    let re_qreg: Regex = Regex::new(r"q\[(\d+)\]").unwrap();
    let re_creg: Regex = Regex::new(r"c\[(\d+)\]").unwrap();

    let reader = BufReader::new(input);

    let mut lines = reader
        .split(b';')
        .map(|res| String::from_utf8(res.unwrap()).unwrap().trim().to_string())
        .filter(|line| !line.is_empty());

    lines
        .next()
        .filter(|line| line == "OPENQASM 2.0")
        .ok_or("openqasm error")?;

    lines
        .next()
        .filter(|line| line == "include \"qelib1.inc\"")
        .ok_or("include error")?;

    let qreg: u32 = match split_head(&lines.next().unwrap()) {
        ("qreg", s) => capture(&re_qreg, s),
        _ => return Err("qreg error"),
    };

    let creg: u32 = match split_head(&lines.next().unwrap()) {
        ("creg", s) => capture(&re_creg, s),
        _ => return Err("creg error"),
    };

    let mut instrs = Vec::new();
    let mut qreg_used = HashSet::new();

    let mut capture_qreg = |s: &str| {
        let i = capture(&re_qreg, s);
        qreg_used.insert(i);
        i
    };

    for line in lines {
        instrs.push(match split_head(&line) {
            ("h", s) => Ins::H(capture_qreg(s)),
            ("x", s) => Ins::X(capture_qreg(s)),
            ("t", s) => Ins::T(capture_qreg(s)),
            ("tdg", s) => Ins::Tdg(capture_qreg(s)),
            ("cx", s) => {
                let mut split = s.split(",");
                Ins::Cx(
                    capture_qreg(split.next().unwrap()),
                    capture_qreg(split.next().unwrap()),
                )
            }
            _ => return Err("invalid instruction"),
        });
    }

    Ok(Prog {
        qreg,
        qreg_used: qreg_used.len() as u32,
        creg,
        instrs,
    })
}
