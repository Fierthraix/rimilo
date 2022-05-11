use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use phf_codegen::Map;
use serde_yaml::{self, Value};

fn main() {
    let jaml_vortoj: Value = serde_yaml::from_str(include_str!("./vortaro/vortaro.yaml")).unwrap();
    let jaml_mapo = match jaml_vortoj {
        Value::Mapping(mapo) => mapo,
        _ => panic!(),
    };

    let mut mapo = Map::new();
    for enigo in &jaml_mapo {
        match enigo {
            (Value::String(ŝlosilo), Value::String(valuo)) => {
                mapo.entry(ŝlosilo, &format!(r#""{}""#, valuo));
            }
            _ => (),
        }
    }

    let ĝenita_dosiervojo = Path::new(&env::var("OUT_DIR").unwrap()).join("vortoj.rs");
    let mut dosiero = BufWriter::new(File::create(&ĝenita_dosiervojo).unwrap());
    writeln!(&mut dosiero, "{}", mapo.build()).unwrap();
}
