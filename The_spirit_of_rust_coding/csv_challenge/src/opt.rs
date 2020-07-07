use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = "Usage")]
pub struct Opt{
    pub input:String,
    pub column_name: String,
    pub replacement: String,
    pub output: Option<String>,
}