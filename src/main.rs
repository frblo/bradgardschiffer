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
}

fn main() {
    let _args = Args::parse();

    match _args.alphabet {
        Some(alpb) => cipherer::encipher(_args.input.to_uppercase(), alpb),
        None => cipherer::std_encipher(_args.input.to_uppercase())
    };
}
