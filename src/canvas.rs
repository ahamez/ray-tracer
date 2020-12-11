// --------------------------------------------------------------------------------------------- //

use crate::color::Color;
use std::fmt::Write;

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

// --------------------------------------------------------------------------------------------- //

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas::new_with_color(width, height, Color::black())
    }

    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn ppm(&self) -> String {
        let mut ppm = String::new();
        self.write_ppm_header(&mut ppm);

        // TODO: Split every 70 characters. See https://stackoverflow.com/a/57032118/21584?
        for row in 0..self.height {
            for col in 0..self.width - 1 {
                let color = &self[row][col];
                let (r, g, b) = scale_color(color);
                write!(ppm, "{} {} {} ", r, g, b).unwrap();
            }

            let color = &self[row][self.width - 1];
            let (r, g, b) = scale_color(color);
            write!(ppm, "{} {} {}", r, g, b).unwrap();

            write!(ppm, "\n").unwrap();
        }

        ppm
    }

    fn write_ppm_header(&self, ppm: &mut String) {
        writeln!(ppm, "P3").unwrap();
        writeln!(ppm, "{} {}", self.width, self.height).unwrap();
        writeln!(ppm, "255").unwrap();
    }
}

// --------------------------------------------------------------------------------------------- //

fn scale_color(color: &Color) -> (u8, u8, u8) {
    (
        scale_color_component(color.r),
        scale_color_component(color.g),
        scale_color_component(color.b),
    )
}

fn scale_color_component(component: f64) -> u8 {
    let component = if component < 0.0 {
        0.0
    } else if component > 1.0 {
        1.0
    } else {
        component
    };

    (component * 255.0) as u8
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Index<usize> for Canvas {
    type Output = [Color];

    fn index(&self, row: usize) -> &[Color] {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl std::ops::IndexMut<usize> for Canvas {
    fn index_mut(&mut self, row: usize) -> &mut [Color] {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pixel() {
        let mut canvas = Canvas::new(10, 20);
        canvas[2][3] = Color::red();

        assert_eq!(canvas[2][3], Color::red());
        assert_eq!(canvas[0][1], Color::black());
    }

    #[test]
    fn write_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let mut ppm = String::new();
        canvas.write_ppm_header(&mut ppm);

        let expected = "P3
5 3
255
";

        assert_eq!(ppm, expected);
    }

    #[test]
    fn ppm() {
        let mut canvas = Canvas::new(5, 3);
        canvas[0][0] = Color {
            r: 1.5,
            g: 0.0,
            b: 0.0,
        };
        canvas[1][2] = Color {
            r: 0.0,
            g: 0.5,
            b: 0.0,
        };
        canvas[2][4] = Color {
            r: -0.5,
            g: 0.0,
            b: 1.0,
        };

        let ppm = canvas.ppm();

        let expected = "P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 127 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
";

        assert_eq!(ppm, expected);
    }
}

// --------------------------------------------------------------------------------------------- //
