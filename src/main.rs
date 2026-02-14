use clap::Parser;

mod app;

#[derive(Parser)]
#[command(name = "img2ascii", about = "Convert an image to ASCII art")]
struct Cli {
  input: String,
  output: Option<String>,
}

fn main() {
  let cli = Cli::parse();
  app::convert(&cli.input as &str);
}
