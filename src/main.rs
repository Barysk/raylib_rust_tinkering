use raylib::prelude::*;
use raylib::ffi::{RenderTexture, LoadRenderTexture};

fn main() {
    
    let (mut rl, thread) = raylib::init()
        .size(240,320)
        //.resizable()
        .title("SSFv3s_2R")
        .build();
    
    rl.set_target_fps(60u32);
    rl.set_window_min_size(240i32, 320i32);
 
    let game_screen_height= 240i32;
    let game_screen_width= 320i32;
    
    let game_screen: RenderTexture = unsafe {
        LoadRenderTexture(game_screen_width, game_screen_height)
    };
    
    let mut position: Vector2 = Vector2::new(rl.get_screen_height() as f32 / 2f32, rl.get_screen_width() as f32 / 2f32);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        if rl.is_key_pressed(KeyboardKey::KEY_F11){
            rl.toggle_fullscreen();
        }

        // UPDATE
        {
            // Updating code here
            if rl.is_key_down(KeyboardKey::KEY_UP){
                position.y -= 100f32 * delta_time;
            }
            if rl.is_key_down(KeyboardKey::KEY_DOWN){
                position.y += 100f32 * delta_time;
            }
            if rl.is_key_down(KeyboardKey::KEY_RIGHT){
                position.x += 100f32 * delta_time;
            }
            if rl.is_key_down(KeyboardKey::KEY_LEFT){
                position.x -= 100f32 * delta_time;
            }
        }
        // DRAW
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::BLACK);
            d.draw_circle(position.x as i32, position.y as i32, 3f32, Color::RED);
            d.draw_text("Hello SSFv3s", 12i32, 12i32, 32i32, Color::RAYWHITE);
        }
    }
}
