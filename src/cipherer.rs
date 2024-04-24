use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashMap;

struct Brädgård {
    dots: usize,
    ceiling: bool,
    floor: bool,
    rwall: bool,
    lwall: bool,
}

pub fn std_encipher(input: String) {
    let alphabet: String = "ABCDEFGHIJKLMNOPRSTUVXYZÅÄÖ".to_owned();
    encipher(input, alphabet);
}

pub fn encipher(input: String, alpb: String) {
    let mut alphabet: HashMap<char, usize> = HashMap::with_capacity(alpb.len());
    for (i, c) in alpb.chars().enumerate() {
        alphabet.insert(c, i);
    }

    for c in input.chars() {
        match alphabet.get(&c) {
            Some(_) => continue,
            None => panic!("The symbol {} is not featured in the alphabet.", c)
        }
    }

    // Set up svg file
    let mut svg = BufWriter::new(File::create("ciphed.svg")
        .expect("Failed to create svg"));

    // I'm sorry.
    svg.write("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\"\"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n"
        .as_bytes())
        .expect("Failed to write to svg file");
    
    // ugly but it is how it is
    svg.write(format!("<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 {width} {width}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xml:space=\"preserve\" xmlns:serif=\"http://www.serif.com/\" style=\"fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;\">\n",
            width = input.len() * 500)
        .as_bytes())
        .expect("Failed to write to svg file");

    svg.write("</svg>".as_bytes()).expect("Failed to write to svg file");
}

fn brädgård_builder(c: char, alphabet: &mut HashMap<char, usize>) -> Brädgård {
    let pos_a: usize = *alphabet.get(&c).unwrap(); // already checked
    let pos: usize = pos_a % 9;

    return Brädgård{
        dots: pos_a / (alphabet.len() / 9),
        ceiling: pos >= 3,
        floor: pos <= 5,
        rwall: (pos + 1) % 3 != 0,
        lwall: (pos + 1) % 3 == 0 || (pos + 1) % 2 == 0,
    };
}