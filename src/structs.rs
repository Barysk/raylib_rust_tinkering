use crate::constants::*;
use raylib::prelude::*;

// // Just for redference, was needed for toggle_fullsreen
// pub struct MonitorResolution{
//     pub width: i32,
//     pub height: i32
// }

// It is better practice to create functions that will manage our private fields
// and give as less as possible of public fields
// But it'll work for now
pub struct Ball {
    pub direction: Vector2,
    pub position: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
    pub sprite: Texture2D,
}

impl Ball {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, img: &Image) -> Self {
        Self {
            direction: Vector2::new(0f32, 0f32),
            position: Vector2::new(SCREEN_WIDTH / 2f32, SCREEN_HEIGHT / 2f32),
            speed: 120f32,
            radius: 5f32,
            color: Color::RED,
            sprite: rl.load_texture_from_image(thread, img).unwrap(),
        }
    }

    pub fn draw(&self, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
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

pub struct BouncingBall {
    pub position: Vector2,
    pub velocity: Vector2,
    pub radius: f32,
    pub color: Color,
}
