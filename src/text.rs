use crate::constants::SCREEN_WIDTH;
use raylib::prelude::*;

pub fn draw_text_center(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        // SCREEN_WIDTH is a constant, so if screen is resizeable, it is better
        // to use d.get_screen_width();
        (SCREEN_WIDTH as i32 / 2i32) - (text_length / 2),
        y,
        font_size,
        color,
    );
}
