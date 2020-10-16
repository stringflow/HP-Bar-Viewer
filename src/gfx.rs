use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const BYTES_PER_PIXEL: u32 = 3;

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub pitch: usize,
    pub pixels: Vec<u8>,
}

pub struct Font<'a> {
    pub bitmap: Bitmap,
    pub charmap: Vec<&'a str>,
    pub character_width: u32,
    pub character_height: u32,
}

pub fn png_load(data: &'static [u8]) -> Bitmap {
    let decoder = png::Decoder::new(data);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut pixels = vec![0; info.buffer_size()];
    reader.next_frame(&mut pixels).unwrap();
    return Bitmap {
        width: info.width,
        height: info.height,
        pitch: info.line_size,
        pixels,
    };
}

pub fn png_write(file: &str, bitmap: &Bitmap) {
    let path = Path::new(file);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, bitmap.width, bitmap.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&bitmap.pixels).unwrap();
}

pub fn bitmap_copy(src: &Bitmap, src_x: u32, src_y: u32, dest: &mut Bitmap, dest_x: u32, dest_y: u32, width: u32, height: u32) {
    let mut src_row = ((src_x + src_y * src.width) * BYTES_PER_PIXEL) as usize;
    let mut dest_row = ((dest_x + dest_y * dest.width) * BYTES_PER_PIXEL) as usize;
    for _y in 0..height {
        let mut src_ptr = src_row;
        let mut dest_ptr = dest_row;

        for _x in 0..width {
            for _ in 0..3 {
                dest.pixels[dest_ptr] = src.pixels[src_ptr];
                src_ptr += 1;
                dest_ptr += 1;
            }
        }

        src_row += src.pitch as usize;
        dest_row += dest.pitch as usize;
    }
}

pub fn bitmap_text(font: &Font, dest: &mut Bitmap, mut dest_x: u32, dest_y: u32, text: &str) {
    let characters_per_row = font.bitmap.width / font.character_width;
    for character in text.chars() {
        if character != ' ' {
            let index = font.charmap.iter().position(|&c| c == character.to_string()).unwrap() as u32;
            let u = index % characters_per_row;
            let v = index / characters_per_row;
            bitmap_copy(&font.bitmap, u * font.character_width, v * font.character_height, dest, dest_x, dest_y, font.character_width, font.character_height);
        }
        dest_x += font.character_width;
    }
}

pub fn bitmap_fill(dest: &mut Bitmap, dest_x: u32, dest_y: u32, width: u32, height: u32, r: u8, g: u8, b: u8) {
    let mut dest_row = ((dest_x + dest_y * dest.width) * BYTES_PER_PIXEL) as usize;
    for _y in 0..height {
        let mut dest_ptr = dest_row;

        for _x in 0..width {
            dest.pixels[dest_ptr] = r;
            dest_ptr += 1;
            dest.pixels[dest_ptr] = g;
            dest_ptr += 1;
            dest.pixels[dest_ptr] = b;
            dest_ptr += 1;
        }

        dest_row += dest.pitch as usize;
    }
}
