use raytracer_rust::core::canvas::Canvas;
use raytracer_rust::core::color::Color;
use raytracer_rust::core::point::Point;
use raytracer_rust::core::vector::Vector;
use raytracer_rust::io::image::make_image;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(proj: &Projectile, env: &Environment) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    let mut proj = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while proj.position.y > 0.0 {
        let x = proj.position.x as usize;
        let y = canvas.height - (proj.position.y as usize) - 1;

        canvas[[x, y]] = Color::RED;

        proj = tick(&proj, &env);
    }

    make_image(&canvas)
        .save("img/projectile.png")
        .expect("error saving image");
}
