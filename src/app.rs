use image::Luma;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Constants
const MAX_DIMENSION: u32 = 500;

pub fn convert(path_to_file: &str) {
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

  create_ascii_file(path_to_file, &grid);
}

fn get_symbol(brightness: Luma<u16>) -> char {
  let chars = [' ', '.', ':', ';', '+', '?', 'c', 'o', '#', '@', '$'];
  let index = (brightness.0[0] as usize * (chars.len() - 1)) / 65535;
  chars[index]
}

fn resize(path_to_file: &str) {
  let image = image::open(path_to_file).unwrap();
  let resized = image.resize(
    MAX_DIMENSION,
    MAX_DIMENSION,
    image::imageops::FilterType::Lanczos3,
  );
  resized.save(path_to_file).unwrap();
}

fn create_ascii_file(path_to_file: &str, grid: &Vec<Vec<char>>) {
  let name = Path::new(path_to_file)
    .file_name()
    .unwrap()
    .to_str()
    .unwrap();
  let mut file = File::create(format!("converted_{}.txt", name)).unwrap();

  for row in grid {
    let line: String = row.iter().collect();
    writeln!(file, "{}", line).unwrap();
  }
}
