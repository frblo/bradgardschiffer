use std::fs::File;
use std::io::{BufWriter, Write};
use unicode_segmentation::UnicodeSegmentation;

pub fn create_key(alphabet: String, f: Option<String>) {
    if alphabet.graphemes(true).count() > 81 {panic!("Too many characters in alphabet, max 81.")}

    let filename = match f {
        Some(x) => format!("{}_key", x),
        None => "brädgårdsnyckel".to_owned(),
    };

    let mut svg = BufWriter::new(File::create(format!("{}.svg", filename))
        .expect("Failed to create svg"));

    svg.write(format!("<svg width=\"{}\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        ((alphabet.graphemes(true).count() / 9) + 1) * 500)
        .as_bytes())
        .expect("Failed to write to svg file");


    for i in 0..=(alphabet.graphemes(true).count() / 9) {
        draw_walls(i, &mut svg);
    }

    for (i, c) in alphabet.chars().enumerate() {
        draw_char(i / 9, c, i % 9, &mut svg)
    }

    svg.write("</svg>".as_bytes()).expect("Failed to write to svg file");
}

fn draw_walls(index: usize, svg: &mut BufWriter<File>) {
    let draw_wall = |i: usize, line: [usize; 4], svg: &mut BufWriter<File>| {
        svg.write(format!("\t<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"20\" stroke-linecap=\"round\"/>\n",
        line[0] + 500 * i, line[1], line[2] + 500 * i, line[3])
        .as_bytes())
        .expect("Failed to write to svg file");
    };

    draw_wall(index, [195, 100, 195, 400], svg);
    draw_wall(index, [305, 100, 305, 400], svg);
    draw_wall(index, [100, 195, 400, 195], svg);
    draw_wall(index, [100, 305, 400, 305], svg);
}

fn draw_char(index: usize, c: char, pos: usize, svg: &mut BufWriter<File>) {
    let get_pos = |pos: usize| -> (usize, usize) {
        let mut coords = (0, 0);

        coords.0 = match (pos + 1) % 3 {
            1 => 140,
            2 => 250,
            _ => 360,
        };

        if pos <= 2 {coords.1 = 170;}
        else if pos <= 5 {coords.1 = 280;}
        else {coords.1 = 390;}

        return coords;
    };

    let coords = get_pos(pos);

    svg.write(format!("\t<text x=\"{}\" y=\"{}\" dominant-baseline=\"middle\" text-anchor=\"middle\" fill=\"black\" style=\"font: bold 80 px\">{}</text>\n",
        coords.0 + 500 * index, coords.1, c)
        .as_bytes())
        .expect("Failed to write to svg file");
}