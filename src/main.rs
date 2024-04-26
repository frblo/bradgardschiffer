mod cipherer;
mod key;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,

    #[arg(short, long)]
    alphabet: Option<String>,

    #[arg(short, long)]
    filename: Option<String>,

    #[arg(short, long, action)]
    key: bool,

    #[arg(short, long, action)]
    png: bool,
}

fn main() {
    let args = Args::parse();

    match args.input {
        Some(inp) => cipherer::encipher(inp.to_uppercase(), args.alphabet.clone(), args.filename.clone()),
        None => println!("No input given")
    }

    if args.key {
        match args.alphabet {
            Some(a) => key::create_key(a, args.filename),
            None => key::create_key("ABCDEFGHIJKLMNOPRSTUVXYZÅÄÖ".to_owned(), args.filename)            
        }
    }
}
