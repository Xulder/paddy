use clap::Subcommand;
use image::{io::Reader as ImageReader, RgbaImage};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Pad a spritesheet by a given amount of pixels
    Pad {
        /// Width of a sprite frame
        width: u32,
        /// Height of a sprite frame
        height: u32,
        /// Amount of padding to add between sprites
        padding: u32,
        /// Path to the spritesheet you want to pad
        #[clap(parse(from_os_str))]
        sheet_path: std::path::PathBuf,
        /// Optional output path. Default: `filename`-`width`x`height`-pad`padding`.png
        #[clap(parse(from_os_str))]
        output_path: Option<std::path::PathBuf>,
    },
}

pub fn pad_spritesheet(
    width: &u32,
    height: &u32,
    padding: &u32,
    sheet_path: &PathBuf,
    output_path: &Option<PathBuf>,
) {
    // The original image specified by image_path
    let img = ImageReader::open(sheet_path)
        .expect(format!("Trouble opening file {}", sheet_path.to_str().unwrap()).as_str())
        .decode()
        .expect(format!("Had an issue decoding the image. Check for file corruption").as_str());

    // Height and width of the image measured in sprite frames.
    let sheet_width = img.width() / width;
    let sheet_height = img.height() / height;

    // New (empty) image with appropriate padding added.
    let mut out = RgbaImage::new(
        img.width() + ((sheet_width - 1) * padding),
        img.height() + ((sheet_height - 1) * padding),
    );

    for x in 0..sheet_width {
        for y in 0..sheet_height {
            // The frame cut from the original will consistently be (x, y) * (width, height)
            let frame = img
                .crop_imm(x * width, y * height, *width, *height)
                .into_rgba8();

            let dim = (x, y);
            match dim {
                (0, 0) => image::imageops::overlay(&mut out, &frame, 0, 0),
                (x, 0) => {
                    image::imageops::overlay(&mut out, &frame, (x * (width + padding)) as i64, 0)
                }
                (0, y) => {
                    image::imageops::overlay(&mut out, &frame, 0, (y * (height + padding)) as i64)
                }
                (x, y) => image::imageops::overlay(
                    &mut out,
                    &frame,
                    (x * (width + padding)) as i64,
                    (y * (height + padding)) as i64,
                ),
            }
        }
    }

    // let outpath = match output_path {
    //     Some(outpath) => outpath.as_os_str().to_str().unwrap().to_string(),
    //     None => format!(
    //         "./{}-{}x{}-pad{}.png",
    //         &sheet_path.file_stem().unwrap().to_str().unwrap(),
    //         width,
    //         height,
    //         padding
    //     ),
    // };

    let outpath = output_path.clone().unwrap_or(
        PathBuf::from_str(
            format!(
                "./{}-{}x{}-pad{}.png",
                &sheet_path.file_stem().unwrap().to_str().unwrap(),
                width,
                height,
                padding,
            )
            .as_str(),
        )
        .unwrap(),
    );

    out.save(&outpath)
        .expect("Something went wrong writing the image file.");

    println!("Padded image `{}` complete.", &outpath.to_str().unwrap());
}
