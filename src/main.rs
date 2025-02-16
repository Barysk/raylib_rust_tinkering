use raylib::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
const GLSL_VERSION: i32 = 330;
#[cfg(target_arch = "wasm32")]
const GLSL_VERSION: i32 = 100;

const SCREEN_WIDTH: f32 = 640f32;
const SCREEN_HEIGHT: f32 = 480f32;
const VERSION_NAME: &str = "Shader introduced";

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

    // Load models and texture
    let mut model_a = unsafe {
        rl.load_model_from_mesh(
            &thread,
            Mesh::gen_mesh_torus(&thread, 0.4, 1.0, 16, 32).make_weak(),
        )
        .unwrap()
    };
    let mut model_b = unsafe {
        rl.load_model_from_mesh(
            &thread,
            Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0).make_weak(),
        )
        .unwrap()
    };
    let mut model_c = unsafe {
        rl.load_model_from_mesh(
            &thread,
            Mesh::gen_mesh_sphere(&thread, 0.5, 32, 32).make_weak(),
        )
        .unwrap()
    };
    let texture = rl
        .load_texture(&thread, "assets/texel_checker.png")
        .unwrap();

    // Assign texture to default model material
    model_a.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    model_b.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();
    model_c.materials_mut()[0].maps_mut()
        [raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
        .texture = *texture.as_ref();

    // Load shader and set up some uniforms
    let mut shader = rl
        .load_shader(
            &thread,
            Some(&format!("shaders/glsl{}/base_lighting.vs", GLSL_VERSION)),
            Some(&format!("shaders/glsl{}/fog.fs", GLSL_VERSION)),
        )
        .unwrap();
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as usize] =
        shader.get_shader_location("matModel");
    shader.locs_mut()[raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize] =
        shader.get_shader_location("viewPos");

    // Ambient light level
    let ambient_loc = shader.get_shader_location("ambient");
    shader.set_shader_value(ambient_loc, Vector4::new(0.2, 0.2, 0.2, 1.0));

    let mut fog_density = 0.15;
    let fog_density_loc = shader.get_shader_location("fogDensity");
    shader.set_shader_value(fog_density_loc, fog_density);

    // NOTE: All models share the same shader
    model_a.materials_mut()[0].shader = *shader.as_ref();
    model_b.materials_mut()[0].shader = *shader.as_ref();
    model_c.materials_mut()[0].shader = *shader.as_ref();

    // Using just 1 point lights
    create_light(
        LightType::LightPoint,
        rvec3(0, 2, 6),
        Vector3::zero(),
        Color::WHITE,
        &mut shader,
    );

    let mut cam_background_3d = Camera3D::perspective(
        Vector3::new(0f32, 10f32, 10f32),
        Vector3::new(0f32, 0f32, 0f32),
        Vector3::new(0f32, 1f32, 0f32),
        45f32,
    );

    // rl.set_camera_mode(
    //     &cam_background_3d,
    //     raylib::consts::CameraMode::CAMERA_ORBITAL,
    // ); // 

    rl.set_target_fps(60); // Set our game to run at 60 frames-per-second

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

        if rl.is_key_pressed(KeyboardKey::KEY_F11) {
            rl.toggle_fullscreen();
        }

        // shader controls
        {
            rl.update_camera(&mut cam_background_3d, CameraMode::CAMERA_ORBITAL); // Update camera, seting an orbital camera mode

            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_F) {
                fog_density += 0.001;
                if fog_density > 1.0 {
                    fog_density = 1.0;
                }
            }

            if rl.is_key_down(raylib::consts::KeyboardKey::KEY_C) {
                fog_density -= 0.001;
                if fog_density < 0.0 {
                    fog_density = 0.0;
                }
            }

            shader.set_shader_value(fog_density_loc, fog_density);

            // Rotate the torus
            model_a.set_transform(&(*model_a.transform() * Matrix::rotate_x(-0.025)));
            model_a.set_transform(&(*model_a.transform() * Matrix::rotate_z(0.012)));

            // Update the light shader with the camera view position
            let loc = shader.locs_mut()
                [raylib::consts::ShaderLocationIndex::SHADER_LOC_VECTOR_VIEW as usize];
            shader.set_shader_value(loc, cam_background_3d.position);
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

        {
            let mut d = d.begin_mode3D(cam_background_3d);

            // Draw the three models
            d.draw_model(&model_a, Vector3::zero(), 1.0, Color::WHITE);
            d.draw_model(&model_b, rvec3(-2.6, 0, 0), 1.0, Color::WHITE);
            d.draw_model(&model_c, rvec3(2.6, 0, 0), 1.0, Color::WHITE);

            for i in (-20..20).step_by(2) {
                d.draw_model(&model_a, rvec3(i, 0, 2), 1.0, Color::WHITE);
            }
        }

        // ENTER 3D MODE
        {
            let mut d3d = d.begin_mode3D(&cam_background_3d);
            d3d.draw_grid(128i32, 4f32);
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

        d.draw_text(VERSION_NAME, 12i32, 12i32, 16i32, Color::RAYWHITE);
        // d.draw_fps(12i32, 32i32);
        d.draw_text(
            &d.get_fps().to_string(),
            12i32,
            24i32,
            16i32,
            Color::RAYWHITE,
        );
        d.draw_text(
            &fog_density.to_string(),
            12i32,
            36i32,
            12i32,
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

/**********/
/* LIGHTS */
/**********/

const MAX_LIGHTS: u32 = 4;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LightType {
    LightDirectional = 0,
    LightPoint = 1,
}

impl Default for LightType {
    fn default() -> Self {
        Self::LightDirectional
    }
}

#[derive(Debug, Default, Clone)]
pub struct Light {
    pub enabled: bool,
    pub light_type: LightType,
    pub position: Vector3,
    pub target: Vector3,
    pub color: Color,
    pub enabled_loc: i32,
    pub type_loc: i32,
    pub pos_loc: i32,
    pub target_loc: i32,
    pub color_loc: i32,
}

static mut LIGHTS_COUNT: i32 = 0;

// Defines a light and get locations from PBR shader
pub fn create_light(
    light_type: LightType,
    pos: Vector3,
    targ: Vector3,
    color: Color,
    shader: &mut Shader,
) -> Light {
    let mut light = Light::default();

    if (unsafe { LIGHTS_COUNT as u32 } < MAX_LIGHTS) {
        light.enabled = true;
        light.light_type = light_type;
        light.position = pos.clone();
        light.target = targ.clone();
        light.color = color.clone();

        let lights_count = unsafe { LIGHTS_COUNT };
        let enabled_name = format!("lights[{}].enabled", lights_count);
        let type_name = format!("lights[{}].type", lights_count);
        let pos_name = format!("lights[{}].position", lights_count);
        let target_name = format!("lights[{}].target", lights_count);
        let color_name = format!("lights[{}].color", lights_count);

        // Set location name [x] depending on lights count
        light.enabled_loc = shader.get_shader_location(&enabled_name);
        light.type_loc = shader.get_shader_location(&type_name);
        light.pos_loc = shader.get_shader_location(&pos_name);
        light.target_loc = shader.get_shader_location(&target_name);
        light.color_loc = shader.get_shader_location(&color_name);

        update_light_values(shader, light.clone());
        unsafe {
            LIGHTS_COUNT += 1;
        }
    }

    return light;
}

pub fn update_light_values(shader: &mut Shader, light: Light) {
    // Send to shader light enabled state and type
    shader.set_shader_value(light.enabled_loc, light.enabled as i32);
    shader.set_shader_value(light.type_loc, light.light_type as i32);

    // Send to shader light position values
    shader.set_shader_value(light.pos_loc, light.position);

    // Send to shader light target position values
    shader.set_shader_value(light.target_loc, light.target);

    // Send to shader light color values
    let color: Vector4 = light.color.into();
    shader.set_shader_value(light.color_loc, color);
}
