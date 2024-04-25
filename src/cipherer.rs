use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    svg.write("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n"
        .as_bytes())
        .expect("Failed to write to svg file");
    
    // ugly but it is how it is
    svg.write(format!("<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 {width} {width}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xml:space=\"preserve\" xmlns:serif=\"http://www.serif.com/\" style=\"fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;\">\n",
            width = input.len() * 500)
        .as_bytes())
        .expect("Failed to write to svg file");

    for (i, c) in input.chars().enumerate() {
        let brgd: Brädgård = brädgård_builder(c, &mut alphabet);

        if brgd.ceiling {
            svg.write(format!("\t<g transform=\"matrix(0,1,-1,0,{},380)\">\n\t\t<path d=\"M70,55L70,400\" style=\"fill:none;stroke:black;stroke-width:40px;\"/>\n\t</g>\n",
                i * 500 + (250 + 60))
                .as_bytes())
                .expect("Failed to write to svg file");
        }
        if brgd.floor {
            svg.write(format!("\t<g transform=\"matrix(0,1,-1,0,{},380)\">\n\t\t<path d=\"M70,55L70,400\" style=\"fill:none;stroke:black;stroke-width:40px;\"/>\n\t</g>\n",
                i as i32 * 500 - 20)
                .as_bytes())
                .expect("Failed to write to svg file");
        }
        if brgd.rwall {
            svg.write(format!("\t<g transform=\"matrix(1,0,0,1,{},-12.8571)\">\n\t\t<path d=\"M70,55L70,400\" style=\"fill:none;stroke:black;stroke-width:40px;\"/>\n\t</g>\n",
                i * 500 + (250 + 60))
                .as_bytes())
                .expect("Failed to write to svg file");
        }
        if brgd.lwall {
            svg.write(format!("\t<g transform=\"matrix(1,0,0,1,{},-12.8571)\">\n\t\t<path d=\"M70,55L70,400\" style=\"fill:none;stroke:black;stroke-width:40px;\"/>\n\t</g>\n",
                i * 500 + (250 - 60))
                .as_bytes())
                .expect("Failed to write to svg file");
        }
    }

    svg.write("</svg>".as_bytes()).expect("Failed to write to svg file");
}

fn brädgård_builder(c: char, alphabet: &mut HashMap<char, usize>) -> Brädgård {
    let pos_a: usize = *alphabet.get(&c).unwrap(); // already checked
    let pos: usize = pos_a % 9;

    return Brädgård {
        dots: pos_a / 9,
        ceiling: pos >= 3,
        floor: pos <= 5,
        rwall: (pos + 1) % 3 != 0,
        lwall: [1, 2, 4, 5, 7, 8].contains(&pos),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brädgård_builder() {
        let alpb = "ABCDEFGHIJKLMNOPRSTUVXYZÅÄÖ";
        let mut alphabet: HashMap<char, usize> = HashMap::with_capacity(alpb.len());
        for (i, c) in alpb.chars().enumerate() {
            alphabet.insert(c, i);
        }

        let a_brädgård: Brädgård = Brädgård {
            dots: 0,
            ceiling: false,
            floor: true,
            rwall: true,
            lwall: false,
        };
        assert_eq!(brädgård_builder('A', &mut alphabet), a_brädgård);

        let e_brädgård: Brädgård = Brädgård {
            dots: 0,
            ceiling: true,
            floor: true,
            rwall: true,
            lwall: true,
        };
        assert_eq!(brädgård_builder('E', &mut alphabet), e_brädgård);

        let i_brädgård: Brädgård = Brädgård {
            dots: 0,
            ceiling: true,
            floor: false,
            rwall: false,
            lwall: true,
        };
        assert_eq!(brädgård_builder('I', &mut alphabet), i_brädgård);

        let n_brädgård: Brädgård = Brädgård {
            dots: 1,
            ceiling: true,
            floor: true,
            rwall: true,
            lwall: true,
        };
        assert_eq!(brädgård_builder('N', &mut alphabet), n_brädgård);

        let å_brädgård: Brädgård = Brädgård {
            dots: 2,
            ceiling: true,
            floor: false,
            rwall: true,
            lwall: false,
        };
        assert_eq!(brädgård_builder('Å', &mut alphabet), å_brädgård);
    }

    #[test]
    #[should_panic]
    fn test_encipher_panic() {
        let alphabet: String = "ABCDEFGHIJ".to_owned();
        encipher("bonanza".to_owned(), alphabet);
    }
}