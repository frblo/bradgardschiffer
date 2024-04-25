mod cipherer;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    alphabet: Option<String>,

    #[arg(short, long)]
    filename: Option<String>,

    #[arg(short, long, action)]
    png: bool,
}

fn main() {
    let args = Args::parse();

    cipherer::encipher(args.input.to_uppercase(), args.alphabet, args.filename)
}
