use std::io::Write;
use std::path::Path;

use crate::create_file;

pub fn read(path: impl AsRef<Path>) {
    let mut file = create_file(path.as_ref());
    let image = image::open(path.as_ref()).unwrap();
    let image = image.as_rgb8().unwrap();
    let width = image.width();
    let height = image.height();
    write!(file, "#[rustfmt::skip]\npub const S: rp_spi_tft::Sprite::<{width},{height}> = rp_spi_tft::Sprite([").unwrap();
    for y in 0..height {
        write!(file, "\n\t[").unwrap();
        for x in 0..width {
            let color = image.get_pixel(x, y);
            let r = ((color.0[0] as f32 / 255.) * 31.) as u16;
            let g = ((color.0[1] as f32 / 255.) * 63.) as u16;
            let b = ((color.0[2] as f32 / 255.) * 31.) as u16;
            let color =
                (r << 11) +
                ((g << 5) & 0b00000_111111_00000) +
                (b & 0b00000_000000_11111);
            write!(file, "{color},").unwrap();
        }
        write!(file, "],").unwrap();
    }
    write!(file, "\n]);").unwrap();
}