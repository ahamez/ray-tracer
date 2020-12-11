// --------------------------------------------------------------------------------------------- //

use crate::color::Color;

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

    pub fn export(&self, path: &str) -> image::ImageResult<()> {
        let mut img = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let color = &self[y as usize][x as usize];
            let (r, g, b) = scale_color(color);
            *pixel = image::Rgb([r, g, b]);
        }

        img.save(path)
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
}

// --------------------------------------------------------------------------------------------- //
