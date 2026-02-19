use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use phf_codegen::OrderedMap;
use serde_yaml::{self, Value};

fn main() {
    let vortara_dosierujo = Path::new("vortaro");
    let mut enigoj = kolekti_enigojn(vortara_dosierujo);

    // Konservu stabilan ordon por rekreado de la sama dosiero.
    enigoj.sort();

    // Kolektu ĉiujn vortoparojn en unu mapon.
    let mut vortoparoj = std::collections::BTreeMap::new();
    for enigo in enigoj {
        println!("cargo:rerun-if-changed={}", enigo.display());

        if let Some(etendaĵo) = enigo.extension().and_then(|e| e.to_str()) {
            match etendaĵo {
                "yaml" => enmeti_el_jaml(&enigo, &mut vortoparoj),
                "json" => enmeti_el_ĵsono(&enigo, &mut vortoparoj),
                _ => {}
            }
        }
    }

    // Enmetu enigojn en la hakettabulon el ĉiuj fontoj.
    let mut mapo = OrderedMap::new();
    for (ŝlosilo, valuo) in vortoparoj {
        mapo.entry(ŝlosilo, &format!(r#""{}""#, valuo));
    }

    // Konservu hakettabulon al disko por aliaj funkcioj.
    let ĝenita_dosiervojo = Path::new(&env::var("OUT_DIR").unwrap()).join("vortoj.rs");
    let mut dosiero = BufWriter::new(File::create(ĝenita_dosiervojo).unwrap());
    writeln!(&mut dosiero, "{}", mapo.build()).unwrap();
}

fn kolekti_enigojn(vortara_dosierujo: &Path) -> Vec<std::path::PathBuf> {
    fs::read_dir(vortara_dosierujo)
        .unwrap()
        .filter_map(|ero| {
            let dosiervojo = ero.ok()?.path();
            let etendaĵo = dosiervojo.extension()?.to_str()?;
            if etendaĵo == "yaml" || etendaĵo == "json" {
                Some(dosiervojo)
            } else {
                None
            }
        })
        .collect()
}

fn enmeti_el_jaml(dosiervojo: &Path, vortoparoj: &mut std::collections::BTreeMap<String, String>) {
    let enhavo = fs::read_to_string(dosiervojo).unwrap();
    let enhavo = forigi_bom(enhavo);
    for (lini_indekso, linio) in enhavo.lines().enumerate() {
        let linio = linio.trim();
        if linio.is_empty() || linio.starts_with('#') {
            continue;
        }

        let dokumento = format!("{{{}}}", linio);
        let valoro: Value = serde_yaml::from_str(&dokumento).unwrap_or_else(|eraro| {
            panic!(
                "Nevalida YAML-linio en {}:{}: {}",
                dosiervojo.display(),
                lini_indekso + 1,
                eraro
            )
        });

        let Value::Mapping(mapo) = valoro else {
            continue;
        };

        for (ŝlosilo, valuo) in mapo {
            if let (Value::String(ŝlosilo), Value::String(valuo)) = (ŝlosilo, valuo) {
                vortoparoj.insert(ŝlosilo, valuo);
            }
        }
    }
}

fn enmeti_el_ĵsono(dosiervojo: &Path, vortoparoj: &mut std::collections::BTreeMap<String, String>) {
    let enhavo = fs::read_to_string(dosiervojo).unwrap();
    let enhavo = forigi_bom(enhavo);
    let valoro: Value = serde_yaml::from_str(&enhavo).unwrap();
    let radiko = match valoro {
        Value::Mapping(mapo) => mapo,
        _ => panic!("Nevalida JSON-radiko en {}", dosiervojo.display()),
    };

    let vortoj = match radiko.get("words") {
        Some(Value::Sequence(vortoj)) => vortoj,
        _ => return,
    };

    for vorto in vortoj {
        let vorto = match vorto {
            Value::Mapping(mapo) => mapo,
            _ => continue,
        };

        let eo_vortoj = legi_vortojn(vorto, "eo");
        let mut en_vortoj = legi_vortojn(vorto, "en");
        en_vortoj.sort();
        en_vortoj.dedup();
        if eo_vortoj.is_empty() || en_vortoj.is_empty() {
            continue;
        }

        let traduko = en_vortoj.join(", ");
        for eo_vorto in eo_vortoj {
            if !eo_vorto.is_empty() {
                vortoparoj.insert(eo_vorto, traduko.clone());
            }
        }
    }
}

fn legi_vortojn(mapo: &serde_yaml::Mapping, lingvo: &str) -> Vec<String> {
    let mut rezultoj = Vec::new();
    let Some(Value::Sequence(vortoj)) = mapo.get(lingvo) else {
        return rezultoj;
    };

    for vorto in vortoj {
        let Value::Mapping(vorto_mapo) = vorto else {
            continue;
        };
        let Some(Value::String(vorto)) = vorto_mapo.get("word") else {
            continue;
        };
        rezultoj.push(vorto.to_string());
    }

    rezultoj
}

fn forigi_bom(mut enhavo: String) -> String {
    if enhavo.starts_with('\u{feff}') {
        enhavo.remove(0);
    }
    enhavo
}
