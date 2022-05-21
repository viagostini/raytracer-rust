use raytracer_rust::core::point::Point;
use raytracer_rust::core::vector::Vector;

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
    let mut p = Projectile {
        position: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        velocity: Vector {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        }
        .normalize(),
    };

    let e = Environment {
        gravity: Vector {
            x: 0.0,
            y: -0.1,
            z: 0.0,
        },
        wind: Vector {
            x: -0.01,
            y: 0.0,
            z: 0.0,
        },
    };

    while p.position.y > 0.0 {
        println!("Current projectile position: {:?}", p.position);
        p = tick(&p, &e);
    }
}
