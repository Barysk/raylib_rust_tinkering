use raylib::prelude::*;
use raylib::ffi::{RenderTexture, LoadRenderTexture};

fn main() {
    
    let (mut rl, thread) = raylib::init()
        .size(240,320)
        .resizable()
        .title("SSFv3s_2R")
        .build();
    
    rl.set_target_fps(60u32);
    rl.set_window_min_size(240i32, 320i32);
 
    let game_screen_height= 240i32;
    let game_screen_width= 320i32;
    
    let game_screen: RenderTexture = unsafe {
        LoadRenderTexture(game_screen_width, game_screen_height)
    };

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_F11){
            rl.toggle_fullscreen();
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello SSFv3s", 12i32, 12i32, 32i32, Color::RAYWHITE);
    }
}
