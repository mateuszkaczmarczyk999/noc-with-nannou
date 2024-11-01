use nannou::prelude::*;
use rand::Rng;

struct Walker {
    position: Point2,
    size: f32,
    color: Rgba,
}

struct Model {
    walkers: Vec<Walker>,
    mouse_pos: Point2
}

fn model(_app: &App) -> Model {
    let mut walkers = Vec::new();
    walkers.push(create_walker(pt2(0.0, 0.0)));
    Model { walkers, mouse_pos: pt2(0.0, 0.0) }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();
    let next_gen: bool = random();
    let next_move: bool = random();
    model.mouse_pos = app.mouse.position();

    if next_gen { model.walkers.push(create_walker(model.mouse_pos)); }

    for walker in &mut model.walkers {
        if next_move {
            rand_move_walker(walker);
        } else {
            target_move_walker(walker, model.mouse_pos);
        }
        bound_walker(walker, boundary);
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

fn create_walker(origin: Point2) -> Walker {
    let mut rand_gen = rand::thread_rng();
    Walker {
        position: origin,
        size: 3.0,
        color: rgba(rand_gen.gen(), rand_gen.gen(), rand_gen.gen(), 0.1),
    }
}

fn bound_walker(walker: &mut Walker, boundary: Rect) {
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

fn target_move_walker(walker: &mut Walker, target: Point2) {
    let direction = (target - walker.position).normalize();
    walker.position += direction;
}

fn rand_move_walker(walker: &mut Walker) {
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
}
