# Image to ASCII CLI

Convert images to ASCII art directly from your terminal.

## Installation

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone this repository:
	```sh
	git clone <repo-url>
	cd image_to_ascii
	```
3. Build the project:
	```sh
	cargo build --release
	```
4. The binary will be in the `target/release` directory.


## Usage

```sh
img2ascii <input_image>
```

- `<input_image>`: Path to the image file you want to convert. (Relative to [Cargo.toml](/Cargo.toml))

### Example

![alt text](example.png)


## License
[MIT](/LICENSE)

---
Made for the 30 Tage 30 CLIs challenge.
