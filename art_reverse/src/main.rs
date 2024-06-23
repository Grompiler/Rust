use image::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() -> Result<()> {
    let file_name = "./vendor.zip";
    let image_name = "vendor.png";
    let file_output_name = "output";
    write_file_bits_to_png_image(file_name, image_name)?;
    reconstruct_file_from_png_image(image_name, file_output_name)
}

fn write_file_bits_to_png_image(file_name: &str, image_name: &str) -> Result<()> {
    let file = File::open(file_name)?;
    let meta = file.metadata()?;
    let meta_len = meta.len() as f64;
    let img_side_len = meta_len.sqrt().ceil() as u32;

    let mut file_bytes = file.bytes();
    let mut img_buffer = ImageBuffer::new(img_side_len, img_side_len);
    let mut rgb_cpt = 0;
    for (_x, _y, pixel) in img_buffer.enumerate_pixels_mut() {
        if (meta.len() - 3) > rgb_cpt {
            if let (Some(r), Some(g), Some(b)) =
                (file_bytes.next(), file_bytes.next(), file_bytes.next())
            {
                *pixel = Rgba([r.unwrap(), g.unwrap(), b.unwrap(), 255]);
            }
            rgb_cpt += 3;
        } else {
            if let Some(r) = file_bytes.next() {
                if let Some(g) = file_bytes.next() {
                    *pixel = Rgba([r.unwrap(), g.unwrap(), 255, 2]);
                } else {
                    *pixel = Rgba([r.unwrap(), 255, 255, 1]);
                }
            } else {
                *pixel = Rgba([255, 255, 255, 0]);
            }
        }
    }

    img_buffer.save(image_name).unwrap();

    Ok(())
}

fn reconstruct_file_from_png_image(image_name: &str, file_output_name: &str) -> Result<()> {
    let png_reader = io::Reader::open(image_name)?.decode().unwrap();
    let mut output = File::create(file_output_name)?;

    for (_x, _y, rgba) in png_reader.pixels() {
        match rgba[3] {
            255 => {
                output.write(&[rgba[0]])?;
                output.write(&[rgba[1]])?;
                output.write(&[rgba[2]])?;
            }
            2 => {
                output.write(&[rgba[0]])?;
                output.write(&[rgba[1]])?;
            }
            1 => {
                output.write(&[rgba[0]])?;
            }
            0 => {}
            _ => panic!("this should not happen"),
        }
    }

    Ok(())
}
