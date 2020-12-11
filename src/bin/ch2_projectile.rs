use ray_tracer::{canvas::Canvas, color::Color, point::Point, tuple::Tuple, vector::Vector};

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
struct Env {
    gravity: Vector,
    wind: Vector,
}

// --------------------------------------------------------------------------------------------- //

fn tick(env: &Env, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + env.gravity + env.wind,
    }
}

// --------------------------------------------------------------------------------------------- //

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Env {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new_with_color(900, 500, Color::black());
    while p.position.y() > 0.0 {
        p = tick(&env, p);
        let row = 500 - (p.position.y() as usize) - 1;
        let col = p.position.x() as usize;
        canvas[row][col] = Color::red();
    }

    canvas.export("./traj.png").unwrap();
}
