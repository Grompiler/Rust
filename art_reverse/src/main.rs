use image::*;

use std::io::prelude::*;

use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("./target/debug/pdf_worker")?;

    let meta = file.metadata()?;

    let img_x = meta.len() as f64;
    let img_x = img_x.sqrt() as u32;

    let img_y = img_x + ((meta.len() as f64 / img_x as f64).ceil() - img_x as f64) as u32;

    let mut file_iter = file.bytes();

    let mut img_buf = ImageBuffer::new(img_x, img_y);

    let mut rgb_cpt = 0;
    for (_x, _y, pixel) in img_buf.enumerate_pixels_mut() {
        if (meta.len() - 3) > rgb_cpt {
            if let (Some(r), Some(g), Some(b)) =
                (file_iter.next(), file_iter.next(), file_iter.next())
            {
                *pixel = Rgba([r.unwrap(), g.unwrap(), b.unwrap(), 255]);
            }
            rgb_cpt += 3;
        } else {
            if let Some(r) = file_iter.next() {
                if let Some(g) = file_iter.next() {
                    *pixel = Rgba([r.unwrap(), g.unwrap(), 255, 2]);
                } else {
                    *pixel = Rgba([r.unwrap(), 255, 255, 1]);
                }
            } else {
                *pixel = Rgba([255, 255, 255, 0]);
            }
        }
    }
    img_buf.save("prog1.png").unwrap();
    println!("x: {}", img_x);
    println!("y: {}", img_y);
    println!("len: {}", meta.len());

    let png_reader = io::Reader::open("prog1.png")?.decode().unwrap();
    let mut output = File::create("pdf_worker_revive")?;
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
