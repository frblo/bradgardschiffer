use std::fs::File;
use std::io::{BufWriter, Write};
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
struct Brädgård {
    dots: usize,
    walls: Vec<Wall>
}

#[derive(Debug, PartialEq)]
enum Wall {
    Top,
    Bottom,
    Right,
    Left,
}

pub fn encipher(input: String, alph: Option<String>, filename: Option<String>) {
    let mut alphabet: HashMap<char, usize> = match &alph {
        Some(a) => HashMap::with_capacity(a.graphemes(true).count()),
        None => HashMap::with_capacity(27) // Standard alphabet
    };

    let alphabet_adder = |alph: String, alphabet: &mut HashMap<char, usize>| {
        for (i, c) in alph.chars().enumerate() {
            match alphabet.get(&c) {
                Some(_) => panic!("Char {} already exists in alphabet", c),
                None => alphabet.insert(c, i)                
            };
        }
    };

    match alph {
        Some(a) => alphabet_adder(a, &mut alphabet),
        None => alphabet_adder("ABCDEFGHIJKLMNOPRSTUVXYZÅÄÖ".to_owned(), &mut alphabet),
    }
    if alphabet.len() > 81 {panic!("Too many characters in alphabet, max 81.")}

    for c in input.chars() {
        match alphabet.get(&c) {
            Some(_) => continue,
            None => panic!("The symbol {} is not featured in the alphabet.", c)
        }
    }

    let f = match filename {
        Some(x) => x,
        None => "ciphed".to_owned(),
    };

    encipherer(input, &mut alphabet, f);
}

fn encipherer(input: String, alphabet: &mut HashMap<char, usize>, filename: String) {
    // Set up svg file
    let mut svg = BufWriter::new(File::create(format!("{}.svg", filename))
        .expect("Failed to create svg"));
    
    svg.write(format!("<svg width=\"{}\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        input.graphemes(true).count() * 500)
        .as_bytes())
        .expect("Failed to write to svg file");

    for (i, c) in input.chars().enumerate() {
        let bg: Brädgård = brädgård_builder(c, alphabet);

        draw_brädgård(i, bg, &mut svg);
    }

    svg.write("</svg>".as_bytes()).expect("Failed to write to svg file");
}

fn brädgård_builder(c: char, alphabet: &mut HashMap<char, usize>) -> Brädgård {
    let pos_a: usize = *alphabet.get(&c).unwrap(); // already checked

    let bg_walls = |pos: usize| -> Vec<Wall> {
        let mut walls: Vec<Wall> = Vec::with_capacity(4);
        if pos >= 3 {walls.push(Wall::Top);}
        if pos <= 5 {walls.push(Wall::Bottom);}
        if (pos + 1) % 3 != 0 {walls.push(Wall::Right);}
        if [1, 2, 4, 5, 7, 8].contains(&pos) {walls.push(Wall::Left);}

        return walls;
    };

    return Brädgård {
        dots: pos_a / 9,
        walls: bg_walls(pos_a % 9)
    };
}

fn draw_brädgård(index: usize, bg: Brädgård, svg: &mut BufWriter<File>) {
    bg.walls.into_iter().for_each(|w| draw_wall(index, w, svg));

    // Draw dots
    match bg.dots {
        1 => draw_dot(index, (250, 250), svg),
        2 => {
            draw_dot(index, (175, 250), svg);
            draw_dot(index, (325, 250), svg);
        },
        3 => {
            draw_dot(index, (133, 250), svg);
            draw_dot(index, (250, 250), svg);
            draw_dot(index, (367, 250), svg);
        },
        4 => {
            draw_dot(index, (175, 175), svg);
            draw_dot(index, (325, 175), svg);
            draw_dot(index, (175, 325), svg);
            draw_dot(index, (325, 325), svg);
        },
        5 => {
            draw_dot(index, (150, 150), svg);
            draw_dot(index, (350, 150), svg);
            draw_dot(index, (250, 250), svg);
            draw_dot(index, (150, 350), svg);
            draw_dot(index, (350, 350), svg);
        },
        6 => {
            draw_dot(index, (175, 133), svg);
            draw_dot(index, (325, 133), svg);
            draw_dot(index, (175, 250), svg);
            draw_dot(index, (325, 250), svg);
            draw_dot(index, (175, 367), svg);
            draw_dot(index, (325, 367), svg);
        }
        7 => {
            draw_dot(index, (150, 133), svg);
            draw_dot(index, (350, 133), svg);
            draw_dot(index, (150, 250), svg);
            draw_dot(index, (250, 250), svg);
            draw_dot(index, (350, 250), svg);
            draw_dot(index, (150, 367), svg);
            draw_dot(index, (350, 367), svg);
        }
        8 => {
            draw_dot(index, (133, 133), svg);
            draw_dot(index, (250, 133), svg);
            draw_dot(index, (367, 133), svg);
            draw_dot(index, (133, 250), svg);
            draw_dot(index, (367, 250), svg);
            draw_dot(index, (133, 367), svg);
            draw_dot(index, (250, 367), svg);
            draw_dot(index, (367, 367), svg);
        }
        9 => {
            draw_dot(index, (133, 133), svg);
            draw_dot(index, (250, 133), svg);
            draw_dot(index, (367, 133), svg);
            draw_dot(index, (133, 250), svg);
            draw_dot(index, (250, 250), svg);
            draw_dot(index, (367, 250), svg);
            draw_dot(index, (133, 367), svg);
            draw_dot(index, (250, 367), svg);
            draw_dot(index, (367, 367), svg);
        }
        _ => ()
    }
}

fn draw_dot(index: usize, pos: (usize, usize), svg: &mut BufWriter<File>) {
    svg.write(format!("\t<circle r=\"50\" cx=\"{}\" cy=\"{}\" fill=\"black\"/>\n",
        pos.0 + 500 * index, pos.1)
        .as_bytes())
        .expect("Failed to write to svg file");
}

fn draw_wall(index: usize, wall: Wall, svg: &mut BufWriter<File>) {
    let line: [usize; 4] = match wall {
        Wall::Top => [50 + index * 500, 50, 450 + index * 500, 50],
        Wall::Bottom => [50 + index * 500, 450, 450 + index * 500, 450],
        Wall::Right => [450 + index * 500, 50, 450 + index * 500, 450],
        Wall::Left => [50 + index * 500, 50, 50 + index * 500, 450],
    };

    // It's too long I know but it is how it is
    svg.write(format!("\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"40\" stroke-linecap=\"round\"/>\n",
        line[0], line[1], line[2], line[3])
        .as_bytes())
        .expect("Failed to write to svg file");
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
            walls: vec![Wall::Bottom, Wall::Right],
        };
        assert_eq!(brädgård_builder('A', &mut alphabet), a_brädgård);

        let e_brädgård: Brädgård = Brädgård {
            dots: 0,
            walls: vec![Wall::Top, Wall::Bottom, Wall::Right, Wall::Left],
        };
        assert_eq!(brädgård_builder('E', &mut alphabet), e_brädgård);

        let i_brädgård: Brädgård = Brädgård {
            dots: 0,
            walls: vec![Wall::Top, Wall::Left],
        };
        assert_eq!(brädgård_builder('I', &mut alphabet), i_brädgård);

        let n_brädgård: Brädgård = Brädgård {
            dots: 1,
            walls: vec![Wall::Top, Wall::Bottom, Wall::Right, Wall::Left],
        };
        assert_eq!(brädgård_builder('N', &mut alphabet), n_brädgård);

        let å_brädgård: Brädgård = Brädgård {
            dots: 2,
            walls: vec![Wall::Top, Wall::Right],
        };
        assert_eq!(brädgård_builder('Å', &mut alphabet), å_brädgård);
    }

    #[test]
    #[should_panic]
    fn test_encipher_panic() {
        let alphabet: String = "ABCDEFGHIJ".to_owned();
        encipher("bonanza".to_owned(), Some(alphabet), None);
    }
}