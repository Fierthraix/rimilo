use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use phf_codegen::OrderedMap;
use serde_yaml::{self, Value};

fn main() {
    // Legu `yaml`-dosieron el disko kiel `Yaml`-strukto.
    let jaml_vortoj: Value = serde_yaml::from_str(include_str!("./vortaro/vortaro.yaml")).unwrap();
    let jaml_mapo = match jaml_vortoj {
        Value::Mapping(mapo) => mapo,
        _ => panic!(),
    };

    // Enmetu enigojn en la hakettabulon el `yaml`.
    let mut mapo = OrderedMap::new();
    for enigo in &jaml_mapo {
        if let (Value::String(ŝlosilo), Value::String(valuo)) = enigo {
            mapo.entry(ŝlosilo, &format!(r#""{}""#, valuo));
        }
    }

    // Konservu hakettabulon al disko por aliaj funkcioj.
    let ĝenita_dosiervojo = Path::new(&env::var("OUT_DIR").unwrap()).join("vortoj.rs");
    let mut dosiero = BufWriter::new(File::create(&ĝenita_dosiervojo).unwrap());
    writeln!(&mut dosiero, "{}", mapo.build()).unwrap();
}
