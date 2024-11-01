use nannou::prelude::*;
use rand::Rng;

struct Walker {
    position: Point2,
    size: f32,
    color: Rgba,
}

struct Model {
    walkers: Vec<Walker>
}

fn model(_app: &App) -> Model {
    let mut walkers = Vec::new();
    walkers.push(create_walker());
    Model { walkers }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();
    let next_gen: bool = random();

    if next_gen { model.walkers.push(create_walker()); }

    for walker in &mut model.walkers {
        move_walker(walker, boundary);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for walker in &model.walkers {
        draw.ellipse()
            .color(walker.color)
            .w_h(walker.size, walker.size)
            .xy(walker.position);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn create_walker() -> Walker {
    let mut rand_gen = rand::thread_rng();
    let x = 0.0;
    let y = 0.0;
    Walker {
        position: pt2(x, y),
        size: 3.0,
        color: rgba(rand_gen.gen(), rand_gen.gen(), rand_gen.gen(), 0.1),
    }
}

fn move_walker(walker: &mut Walker, boundary: Rect) {
    let mut rand_gen = rand::thread_rng();
    let direction = rand_gen.gen_range(1..=8);

    match direction {
        1 => { 
            walker.position.x -= walker.size;
        }
        2 => {
            walker.position.x -= walker.size;
            walker.position.y += walker.size;
        }
        3 => { 
            walker.position.y += walker.size;
        }
        4 => {
            walker.position.x += walker.size;
            walker.position.y += walker.size;
        }
        5 => {
            walker.position.x += walker.size;
        }
        6 => {
            walker.position.x += walker.size;
            walker.position.y -= walker.size;
        }
        7 => {
            walker.position.y -= walker.size;
        }
        8 => {
            walker.position.x -= walker.size;
            walker.position.y -= walker.size;
        }
        _ => {}
    }

    if walker.position.x < boundary.left() {
        walker.position.x += walker.size;
    } else if walker.position.x > boundary.right() {
        walker.position.x -= walker.size;
    };

    if walker.position.y < boundary.bottom() {
        walker.position.y += walker.size;
    } else if walker.position.y > boundary.top() {
        walker.position.y -= walker.size;
    };
}
