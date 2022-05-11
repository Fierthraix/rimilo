use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "Rimilo")]
pub(crate) struct Argar {
    #[structopt(short = "a", long = "angla")]
    pub(crate) angla: bool,

    #[structopt(short = "p", long = "plena")]
    pub(crate) plena: bool,

    #[structopt(short = "f", long = "fina")]
    pub(crate) fina: bool,

    #[structopt()]
    pub(crate) regekso: String,
}
