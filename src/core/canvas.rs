use std::ops::{Index, IndexMut};

use crate::core::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas::new_filled(width, height, Color::BLACK)
    }

    pub fn new_filled(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![color; height]; width],
        }
    }
}

impl Index<[usize; 2]> for Canvas {
    type Output = Color;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;

        &self.pixels[i][j]
    }
}

impl IndexMut<[usize; 2]> for Canvas {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Color {
        let [i, j] = index;

        &mut self.pixels[i][j]
    }
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 10);

        for pixel in canvas.pixels.iter().flatten() {
            assert_eq!(*pixel, Color::BLACK);
        }
    }

    #[test]
    fn create_canvas_filled() {
        let color = Color {
            red: 0.25,
            green: 0.125,
            blue: 0.42,
        };
        let canvas = Canvas::new_filled(10, 10, color);

        for pixel in canvas.pixels.iter().flatten() {
            assert_eq!(*pixel, color);
        }
    }

    #[test]
    fn write_pixel() {
        let mut c = Canvas::new(10, 10);

        c[[5, 5]] = Color::WHITE;

        assert_eq!(c[[5, 5]], Color::WHITE);
    }
}
