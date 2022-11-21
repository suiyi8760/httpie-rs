use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: Subcmd,
}

#[derive(Subcommand,Debug)]
enum Subcmd {}

fn main() {
    let opts = Opts::parse(); 
    println!("{:?}", opts);
}
