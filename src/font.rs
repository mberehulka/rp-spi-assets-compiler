use std::{path::Path, io::Write};

use serde::Deserialize;

use crate::create_file;

fn _default_font_size() -> f32 { 10. }
fn _zero() -> i32 { 0 }

#[derive(Deserialize)]
pub struct Settings {
    #[serde(default = "_default_font_size")]
    font_size: f32,
    #[serde(default = "_zero")]
    offset_x: i32,
    #[serde(default = "_zero")]
    offset_y: i32,
    #[serde(default = "_zero")]
    line_space: i32,
    #[serde(default = "_zero")]
    word_space: i32
}

pub fn read(path: impl AsRef<Path>, settings: &serde_json::Value) {
    let path_name = path.as_ref().file_name().unwrap().to_str().unwrap();
    let settings: Settings = Settings::deserialize(settings.get(path_name).unwrap()).unwrap();
    
    let mut file = create_file(path.as_ref());
    
    let font = std::fs::read(path).unwrap();
    let font = fontdue::Font::from_bytes(font, fontdue::FontSettings {
        scale: settings.font_size,
        ..Default::default()
    }).unwrap();

    let block_letter_metrics = font.metrics('█', settings.font_size);
    let line_height = block_letter_metrics.height as i32 + 1 + settings.line_space;
    let word_space = (block_letter_metrics.width as i32 / 2) + settings.word_space;
    
    file.write(format!("pub const F: rp_spi_tft::Font = rp_spi_tft::Font {{
    line_height: {},
    data: |c| match c {{", line_height).as_bytes()).unwrap();
    
    for c in [
        "abcdefghijklmnopqrstuvxywz",
        "ABCDEFGHIJKLMNOPQRSTUVXYWZ",
        "0123456789",
        "!@#$&%*,.;:?-+",
        // "()_={}[]<>^\\|/\"'",
        // "°¹²³∛√∜∫×≠±⋜⋝ƒ∑∞"
    ].join("").chars() {
        let (metrics, bitmap) = font.rasterize(c, settings.font_size);
        let letter_width = metrics.width;

        assert!(letter_width <= 32, "Letter can not be bigger than 32 pixels");
        assert!(metrics.height <= 32, "Letter can not be bigger than 32 pixels");

        file.write(format!("
        {c:?} => ({letter_width}, &[").as_bytes()).unwrap();

        for y in 0..metrics.height {
            for x in 0..metrics.width {
                let color = ((bitmap[x + y * metrics.width] as f32 / 255.) * 64.) as u16;
                if color > 0 {
                    let dx = x as i32 + settings.offset_x;
                    if dx < 0 { continue }
                    assert!(dx <= 32, "Letter can not be bigger than 32 pixels");

                    let dy = y as i32 + line_height - metrics.height as i32 - metrics.ymin + settings.offset_y;
                    if dy < 0 { continue }
                    assert!(dy <= 32, "Letter can not be bigger than 32 pixels");

                    let v = ((x as u16) << 11) + ((dy as u16) << 6) + color;
                    
                    file.write(format!("{v},").as_bytes()).unwrap();
                }
            }
        }
        file.write(format!("]),").as_bytes()).unwrap();
    }

    file.write(format!("
        ' ' => ({word_space}, &[]),
        _ => (0, &[]),
    }}
}};").as_bytes()).unwrap();
}