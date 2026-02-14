use image::{Luma};
use std::path::{Path};

fn convert(path_to_file: &str) {
    let image = image::open(path_to_file).unwrap().to_luma16();

    let (width, height) = image.dimensions();

    if width > 500 {
        resize(path_to_file);
    }

    let mut grid: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let brightness = image.get_pixel(x, y);
            grid[y as usize][x as usize] = get_symbol(*brightness);
        }
    }

}

fn get_symbol(brightness: Luma<u16>) -> char {
    let chars = [
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 
        'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', 
        ']', '[', '}', '{', '1', ')', '(', '|', '/', 't', 
        'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 
        'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 
        'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', 
        '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'
    ];
    let index = (brightness.0[0] as usize * (chars.len() - 1)) / 65535;
    chars[index]
}

fn resize(path_to_file: &str) {
    let image = image::open(path_to_file).unwrap();
    let resized = image.resize(500, 500, image::imageops::FilterType::Lanczos3);
    resized.save(path_to_file).unwrap();
}

fn into_png(grid: Vec<Vec<char>>) {

}

fn main() {
    convert("image.jpg");
}