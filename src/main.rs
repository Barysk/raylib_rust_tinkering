use std::ops::Range;

use raylib::{ffi::MeasureText, prelude::*};

const SCREEN_WIDTH: f32 = 640f32;
const SCREEN_HEIGHT: f32 = 480f32;
const VERSION_NAME: &str = "Random values";

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
        .title(VERSION_NAME)
        .vsync()
        .build();

    rl.set_target_fps(60u32);

    let mut value: i32 = rl.get_random_value(-100i32..100i32); // not right documentation
    let mut frame_count = 0;

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

        // Example of text appearing
        {
            frame_count += 1;
            if frame_count % 60 == 0 {
                value = rl.get_random_value(-100..100);
                frame_count = 0;
            }
        }

        // Handle ball movement [Mouse]
        {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                ball.position = ball.position.lerp(rl.get_mouse_position(), 0.025f32);
                // you may want to add a desired location to which object will move
            }
        }

        // Handle ball movement [KEYBOARD]
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

        // centered text drawing
        {
            draw_text_center(
                &mut d,
                "every 60 frames new value genrated",
                SCREEN_HEIGHT as i32 / 2i32 - 40i32,
                24i32,
                Color::WHITE,
            );
            draw_text_center(
                &mut d,
                &value.to_string(),
                SCREEN_HEIGHT as i32 / 2i32 - 20i32,
                24i32,
                Color::WHITE,
            );
        }

        // Handle ball drawing
        {
            d.draw_circle_v(ball.position, ball.radius + 2f32, ball.color);
            d.draw_circle_v(ball.position, ball.radius, Color::WHITE);
        }

        d.clear_background(Color::BLACK);
        d.draw_text(VERSION_NAME, 12i32, 12i32, 24i32, Color::RAYWHITE);
    }
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32 / 2i32) - (text_length / 2),
        y,
        font_size,
        color,
    );
}
