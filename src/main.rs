mod argaro;
use structopt::StructOpt;

use regex::Regex;

static VORTAR: phf::Map<&'static str, &'static str> =
    include!(concat!(env!("OUT_DIR"), "/vortoj.rs"));

fn main() {
    let argar = argaro::Argar::from_args();

    let regekso = if argar.fina {
        Regex::new(&format!(r"(?P<unua>{}).$", argar.regekso)).unwrap()
    } else if argar.plena {
        Regex::new(&format!(r"^(?P<unua>{}).$", argar.regekso)).unwrap()
    } else {
        Regex::new(&format!(r"(?P<unua>{})", argar.regekso)).unwrap()
    };

    if argar.angla {
        for (ŝlosilo, valuo) in &VORTAR {
            if regekso.is_match(valuo) {
                println!(
                    "{}: {}",
                    ŝlosilo,
                    regekso.replace(valuo, "\u{1b}[1;31m${1}\u{1b}[0m"),
                );
            }
        }

    } else {
        for (ŝlosilo, valuo) in &VORTAR {
            if regekso.is_match(ŝlosilo) {
                println!(
                    "{}: \t\t{:>3}",
                    regekso.replace(ŝlosilo, "\u{1b}[1;31m${1}\u{1b}[0m"),
                    valuo
                );
            }
        }
    }
}
