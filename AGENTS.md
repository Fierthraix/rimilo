# AGENTS.md

## Lingva Regulo
En ĉi tiu deponejo oni skribu kaj uzu nur Esperanton.

## Deviga Stilo
Ĉiuj komentoj, variabloj, nomoj, komit-mesaĝoj, dokumentado, kaj similaj tekstoj devas esti en ĝusta, gramatike korekta Esperanto.

## Noto
Se iu teknika termino ne havas klaran Esperantan formon, uzu Esperantan priskribon anstataŭ nerekta mikso de lingvoj.

## Strukturo de la kodbazo
- `src/main.rs`: ĉefa lanĉpunkto de la programo; ĝi legas argumentojn, konstruas serĉan regekson kaj montras kongruojn el la vortaro.
- `src/argaro.rs`: difino de la komandliniaj argumentoj (`Argar`) per `structopt`.
- `build.rs`: konstru-skripto kiu legas `vortaro/vortaro.yaml` kaj generas Rust-dosieron `vortoj.rs` en la provizora konstrudirektujo (`OUT_DIR`).
- `vortaro/`: kolekto de vortardatumoj.
- `vortaro/vortaro.yaml`: ĉefa vortara fonto uzata rekte dum la konstru-paŝo.
- `vortaro/frazaro.yaml`, `vortaro/kunmetitaj.yaml`, `vortaro/diff.vortaro.yaml`, `vortaro/asdf.yaml`: aldonaj aŭ helpaj vortolistoj por vortara laborfluo.
- `vortaro/data.json`: aparta datumdosiero por vortara materialo.
- `Cargo.toml`: projekta metadatumo, dependecoj kaj konstru-rilataj agordoj.
- `Cargo.lock`: ŝlositaj versioj de dependecoj por reproduktebla konstruado.

## Fluoskizo
1. La konstru-skripto (`build.rs`) legas `vortaro/vortaro.yaml`.
2. Ĝi transformas la enigojn al statika mapo kaj skribas generitan dosieron `vortoj.rs` en `OUT_DIR`.
3. La ĉefa programo (`src/main.rs`) inkluzivas tiun generitan mapon kaj faras serĉojn laŭ la uzantaj argumentoj.
