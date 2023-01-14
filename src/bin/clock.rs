use std::f64::consts::PI;

use raytracer_rust::{
    core::{canvas::Canvas, color::Color, point::Point, transforms::Transforms},
    io::image::make_image,
};

fn main() {
    let center = Point::new(0.0, 1.0, 0.0);
    let translation = Transforms::translation(50.0, 50.0, 0.0);
    let scaling = Transforms::scaling(30.0, 30.0, 0.0);
    let mut canvas = Canvas::new(100, 100);

    for h in 1..13 {
        let p = (translation * scaling * Transforms::rotation_z(PI / 6.0 * h as f64)) * center;
        canvas.set_pixel(p.x as usize, p.y as usize, Color::WHITE);
    }

    make_image(&canvas)
        .save("img/clock.png")
        .expect("error saving image");
}
