use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "extism-py", about = "Extism Python PDK compiler")]
pub struct Options {
    #[structopt(parse(from_os_str))]
    pub input_py: PathBuf,

    // #[structopt(parse(from_os_str))]
    // pub other_files: Vec<PathBuf>,
    #[structopt(short = "o", parse(from_os_str), default_value = "index.wasm")]
    pub output: PathBuf,

    #[structopt(short = "c")]
    pub core: bool,
}
