use nannou::prelude::*;

struct Model {
    position: Point2,
    size: f32
}

fn model(_app: &App) -> Model {
    Model {
        position: pt2(0.0, 0.0),
        size: 10.0
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();
    let rand_dir_x: bool = random();
    let rand_dir_y: bool = random();

    model.position.x += if rand_dir_x { model.size } else { -model.size };
    model.position.y += if rand_dir_y { model.size } else { -model.size };

    if model.position.x < boundary.left() {
        model.position.x += model.size;
    } else if model.position.x > boundary.right() {
        model.position.x -= model.size;
    };

    if model.position.y < boundary.bottom() {
        model.position.y += model.size;
    } else if model.position.y > boundary.top() {
        model.position.y -= model.size;
    };
}

fn view(app: &App, model: &Model, frame: Frame) {
    // frame.clear(PURPLE);

    let draw = app.draw();
    draw.ellipse().color(RED).w_h(model.size, model.size).xy(model.position);

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
