use macroquad::prelude::*;

// https://mq.agical.se/index.html
//
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}

#[macroquad::main("Bloop")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() * 0.5,
        y: screen_height() * 0.5,
    };

    loop {
        clear_background(DARKBLUE);
        let delta_time = get_frame_time();

        // Inputs
        if is_key_down(KeyCode::Right) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        // generation new squares
        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size * 0.5, screen_width() - size * 0.5),
                y: -size,
            })
        }

        // move squares and remove those out of screen
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        squares.retain(|square| square.y < screen_height() + square.size);

        // Render
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for square in &squares {
            draw_rectangle(
                square.x - square.size * 0.5,
                square.y - square.size * 0.5,
                square.size,
                square.size,
                GREEN,
            );
        }
        next_frame().await
    }
}
