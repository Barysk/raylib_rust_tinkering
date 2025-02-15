use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640f32;
const SCREEN_HEIGHT: f32 = 480f32;

struct Ball {
    direction: Vector2,
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Keyboard input")
        .vsync()
        .build();

    let mut ball = Ball {
        direction: Vector2::new(0f32, 0f32),
        position: Vector2::new(SCREEN_WIDTH / 2f32, SCREEN_HEIGHT / 2f32),
        speed: 120f32,
        radius: 5f32,
        color: Color::RED,
    };

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        /* --- UPDATE --- */

        // Handle ball movement
        {
            let mut direction: Vector2 = Vector2::new(0f32, 0f32);
            if rl.is_key_down(KeyboardKey::KEY_UP) {
                direction.y -= 1f32;
            }
            if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                direction.y += 1f32;
            }
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {
                direction.x -= 1f32;
            }
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
                direction.x += 1f32;
            }
            direction.normalize();
            
            // saving direction into struct to use when drawing sprite
            // struct may not have own direction, if will not be used further
            ball.direction = direction; 

            ball.position += ball.direction * ball.speed * delta_time;
        }

        /* --- DRAW --- */
        let mut d = rl.begin_drawing(&thread);

        // Handle ball drawing
        {
            d.draw_circle_v(ball.position, ball.radius + 2f32, ball.color);
            d.draw_circle_v(ball.position, ball.radius, Color::WHITE);
        }

        d.clear_background(Color::BLACK);
        d.draw_text("Hello world!", 12i32, 12i32, 24i32, Color::RAYWHITE);
    }
}
