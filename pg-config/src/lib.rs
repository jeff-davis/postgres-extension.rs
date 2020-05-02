use std::process::Command;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct PGConfig {
    pub version_num: (u32,u32),
    pub version_string: String,
    pub bindir: String,
    pub pkglibdir: String,
}

fn parse_version(s: &str) -> (u32, u32) {
    let mut next: bool = false;
    for word in s.split_whitespace() {
        if next {
            let pair = Vec::from_iter(word.split("."));
            return (pair[0].parse::<u32>().unwrap(),
                    pair[1].parse::<u32>().unwrap());
        }
        if word.eq("PostgreSQL") {
            next = true;
        }
    }
    panic!("can't parse version string")
}

pub fn pg_config() -> PGConfig {
    let cmd = Command::new("pg_config")
        .args(&[
            "--version",
            "--bindir",
            "--pkglibdir",
        ])
        .output()
        .expect("failed to run pg_config");
    let output = String::from_utf8(cmd.stdout).unwrap();
    let lines: Vec<&str> = Vec::from_iter(output.lines());

    PGConfig {
        version_num: parse_version(lines[0]),
        version_string: lines[0].to_string(),
        bindir: lines[1].to_string(),
        pkglibdir: lines[2].to_string(),
    }
}
