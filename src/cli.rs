use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub day: usize,
    #[arg(short, long)]
    pub challenge: usize,
    #[arg(long, default_value_os_t = PathBuf::from("data"))]
    pub data_path: PathBuf,
}
