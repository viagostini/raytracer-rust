use crate::core::canvas::Canvas;

type Image = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub fn make_image(canvas: &Canvas) -> Image {
    let mut buffer = image::ImageBuffer::new(canvas.width as u32, canvas.height as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let color = canvas[[x as usize, y as usize]];
        *pixel = color.to_rgb();
    }

    buffer
}

#[cfg(test)]
mod image_tests {
    use crate::core::color::Color;

    use super::*;

    #[test]
    fn canvas_to_image() {
        let mut c = Canvas::new(10, 10);

        let mut image = make_image(&c);

        for pixel in image.pixels() {
            assert_eq!(*pixel, image::Rgb([0, 0, 0]));
        }

        c[[5, 5]] = Color::WHITE;

        image = make_image(&c);

        assert_eq!(*image.get_pixel(5, 5), image::Rgb([255, 255, 255]));
    }
}
