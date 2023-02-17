use std::{path::Path, fs::File, io::Write};

use serde::Deserialize;

use crate::create_file;

fn _zero() -> usize { usize::MAX }

#[derive(Deserialize)]
struct Settings {
    #[serde(default = "_zero")]
    max_frames: usize
}

pub fn read(path: impl AsRef<Path>, settings: &serde_json::Value) {
    let path_name = path.as_ref().file_name().unwrap().to_str().unwrap();
    let settings: Settings = Settings::deserialize(settings.get(path_name).unwrap()).unwrap();

    let mut file = create_file(path.as_ref());
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info(File::open(path.as_ref()).unwrap()).unwrap();

    let width = decoder.width() as usize;
    let height = decoder.height() as usize;

    let mut frames = Vec::new();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        if frames.len() >= settings.max_frames { break }
        if frame.width as usize != width || frame.height as usize != height { continue }
        frames.push(frame.clone())
    }

    let frames_length = frames.len();
    write!(file, "#[rustfmt::skip]\npub const S: [rp_spi_tft::Sprite::<{width},{height}>;{frames_length}] = [").unwrap();

    for frame in frames {
        write!(file, "\n\trp_spi_tft::Sprite([").unwrap();
        for y in 0..height as usize {
            write!(file, "\n\t[").unwrap();
            for x in 0..width {
                let i = (x + (y * width)) * 4;
                let r = ((frame.buffer[i] as f32 / 255.) * 31.) as u16;
                let g = ((frame.buffer[i+1] as f32 / 255.) * 63.) as u16;
                let b = ((frame.buffer[i+2] as f32 / 255.) * 31.) as u16;
                let color = (r << 11) + ((g << 5) & 0b00000_111111_00000) + (b & 0b00000_000000_11111);
                write!(file, "{color},").unwrap();
            }
            write!(file, "],").unwrap();
        }
        write!(file, "\n\t]),").unwrap();
    }

    write!(file, "\n];").unwrap();
}