use ndarray::prelude::*;
use ndarray::Zip;

fn main() {
    let input = include_str!("../../puzzle_inputs/day_20.txt").trim();
    let (lookup_table, image) = parse_input(input);
    let mut image = grow(image, 50);

    for i in 0..50 {
        image = enhance(image, &lookup_table);
        if i == 1 {
            println!("20a: {} (5306)", lit_pixels(&image));
        } else if i == 49 {
            println!("20b: {} (17497)", lit_pixels(&image));
        }
    }
}

type Image = Array2<bool>;
type LookupTable = Array1<bool>;

fn grow(image: Image, runs: usize) -> Image {
    let (w, h) = image.dim();
    let mut bigger_image = Array2::from_elem((w + 4 * runs, h + 4 * runs), false);
    let slice = s![(2 * runs)..(2 * runs + w), (2 * runs)..(2 * runs + h)];
    bigger_image.slice_mut(slice).assign(&image);
    bigger_image
}

fn enhance(image: Image, lookup_table: &LookupTable) -> Image {
    Zip::from(image.windows((3, 3))).map_collect(|window| {
        let to_binary = |indx, (place, &digit)| indx | (digit as usize) << (8 - place);
        let indx = window.iter().enumerate().fold(0, to_binary);
        lookup_table[indx]
    })
}

fn lit_pixels(image: &Image) -> usize {
    image.iter().filter(|&&x| x).count()
}

fn parse_array(lines: impl Iterator<Item = &'static str>) -> Array1<bool> {
    let is_lit = |c| c == '#';
    lines.flat_map(|line| line.chars()).map(is_lit).collect()
}

fn parse_input(input: &'static str) -> (LookupTable, Image) {
    let (lookup_table_str, image_str) = input.split_once("\n\n").unwrap();

    // Parse the lookup table
    let lookup_table = parse_array(lookup_table_str.lines());

    // Parse the image
    let mut lines = image_str.lines().peekable();
    let w = lines.peek().unwrap().len();
    let image = parse_array(lines);
    let h = image.len() / w;
    let image = image.into_shape((w, h)).unwrap();

    (lookup_table, image)
}
