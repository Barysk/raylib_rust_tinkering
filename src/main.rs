use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640f32;
const SCREEN_HEIGHT: f32 = 480f32;
const VERSION_NAME: &str = "3D scene introduced";

struct Ball {
    direction: Vector2,
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
    sprite: Texture2D,
}

impl Ball {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, filepath: &str) -> Self {
        Self {
            direction: Vector2::new(0f32, 0f32),
            position: Vector2::new(SCREEN_WIDTH / 2f32, SCREEN_HEIGHT / 2f32),
            speed: 120f32,
            radius: 5f32,
            color: Color::RED,
            sprite: rl.load_texture(thread, filepath).unwrap(),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(
            &self.sprite,
            self.position
                - Vector2::new(
                    self.sprite.width() as f32 / 2f32,
                    self.sprite.height() as f32 / 2f32,
                ),
            self.color,
        );
    }
}

struct BouncingBall {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title(VERSION_NAME)
        .resizable()
        .vsync()
        .build();

    rl.set_target_fps(60u32);

    // Loading shader
    let shader: Shader = rl.load_shader(&thread, Some("shaders/lighting.vs"), Some("shaders/fog.fs")).expect("Failed_to_load");

    let cam_background_3d = Camera3D::perspective(
        Vector3::new(0f32, 10f32, 10f32),
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 1f32, 0f32),
        45f32,
    );

    let mut value: i32 = rl.get_random_value(-100i32..100i32); // not right documentation
    let mut frame_count = 0;
    let mut is_colliding: bool;

    let mut ball = Ball::new(&mut rl, &thread, "assets/tree_left.png");

    let mut bouncing_ball = BouncingBall {
        position: Vector2::new(SCREEN_WIDTH / 2f32, SCREEN_HEIGHT / 2f32),
        velocity: Vector2::new(200f32, 200f32),
        radius: 5f32,
        color: Color::BLUE,
    };

    let texture_ground = rl.load_texture(&thread, "assets/ground.png").unwrap();
    let texture_tree = rl.load_texture(&thread, "assets/tree_right.png").unwrap();

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        
        if rl.is_key_pressed(KeyboardKey::KEY_F11){
            rl.toggle_fullscreen();
        }

        /* --- UPDATE --- */

        // Checking collisios
        {
            if check_collision_circles(
                ball.position,
                ball.radius,
                bouncing_ball.position,
                bouncing_ball.radius,
            ) {
                bouncing_ball.velocity *= -1f32;
                is_colliding = true;
            } else {
                is_colliding = false;
            }
        }

        // Bouncing Ball
        {
            bouncing_ball.position += bouncing_ball.velocity * delta_time;

            if bouncing_ball.position.x >= SCREEN_WIDTH - bouncing_ball.radius
                || bouncing_ball.position.x <= bouncing_ball.radius
            {
                bouncing_ball.velocity.x *= -1f32;
            }
            if bouncing_ball.position.y >= SCREEN_HEIGHT - bouncing_ball.radius
                || bouncing_ball.position.y <= bouncing_ball.radius
            {
                bouncing_ball.velocity.y *= -1f32;
            }
        }

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
        if !is_colliding {
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
        d.clear_background(Color::BLACK);

        // ENTER 3D MODE
        {
            let mut d3d = d.begin_mode3D(&cam_background_3d);
            d3d.draw_cube_v(
                Vector3::new(0f32, 0f32, 0f32),
                Vector3::new(5f32, 5f32, 5f32),
                Color::ORANGE,
            );
            d3d.draw_cube_wires(Vector3::new(0f32, 0f32, 0f32), 5f32, 6f32, 7f32, Color::RED);
            d3d.draw_grid(128i32, 16f32);
        }
        // draw texture
        d.draw_texture_rec(
            &texture_ground,
            Rectangle::new(0f32, 0f32, SCREEN_WIDTH / 4f32, SCREEN_HEIGHT / 4f32),
            Vector2::new(0f32, 0f32),
            Color::WHITE,
        );
        d.draw_texture_v(&texture_tree, Vector2::new(0f32, 0f32), Color::WHITE);

        // draw bouncing ball
        {
            d.draw_circle_v(
                bouncing_ball.position,
                bouncing_ball.radius,
                bouncing_ball.color,
            );
            d.draw_circle_v(
                bouncing_ball.position,
                bouncing_ball.radius - 2f32,
                Color::WHITE,
            );
        }

        // centered text drawing
        {
            draw_text_center(
                &mut d,
                "every 60 frames new value genrated",
                SCREEN_HEIGHT as i32 / 2i32 - 40i32,
                24i32,
                Color::DARKGRAY,
            );
            draw_text_center(
                &mut d,
                &value.to_string(),
                SCREEN_HEIGHT as i32 / 2i32 - 20i32,
                24i32,
                Color::DARKGRAY,
            );
        }

        // Handle ball drawing
        {
            ball.draw(&mut d);
            d.draw_circle_v(ball.position, ball.radius + 2f32, ball.color);
            d.draw_circle_v(ball.position, ball.radius, Color::WHITE);
        }

        d.draw_text(VERSION_NAME, 12i32, 12i32, 24i32, Color::RAYWHITE);
        // d.draw_fps(12i32, 32i32);
        d.draw_text(
            &d.get_fps().to_string(),
            12i32,
            32i32,
            24i32,
            Color::RAYWHITE,
        );
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
