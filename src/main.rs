use macroquad::prelude::*;

// https://mq.agical.se/index.html
//
struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}
impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size * 0.5,
            y: self.y - self.size * 0.5,
            w: self.size,
            h: self.size,
        }
    }
}

#[macroquad::main("Bloop")]
async fn main() {
    const MOVEMENT_SPEED: f32 = 200.0;

    let mut gameover = false;
    let mut squares = vec![];
    let mut bullets: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() * 0.5,
        y: screen_height() * 0.5,
        collided: false,
    };

    loop {
        clear_background(DARKBLUE);
        let delta_time = get_frame_time();
        if !gameover {
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

            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    collided: false,
                })
            }

            // generation new squares
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size * 0.5, screen_width() - size * 0.5),
                    y: -size,
                    collided: false,
                })
            }

            // move squares and remove those out of screen
            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
            // Collisions
            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                    }
                }
            }
            squares.retain(|square| square.y < screen_height() + square.size);
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size * 0.5);
            bullets.retain(|bullet| !bullet.collided);
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() * 0.5;
            circle.y = screen_height() * 0.5;
            gameover = false;
        }
        // Render
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size * 0.5, RED);
        }
        for square in &squares {
            draw_rectangle(
                square.x - square.size * 0.5,
                square.y - square.size * 0.5,
                square.size,
                square.size,
                GREEN,
            );
        }
        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() * 0.5 - text_dimensions.width * 0.5,
                screen_height() * 0.5,
                50.5,
                RED,
            );
        }

        next_frame().await
    }
}
