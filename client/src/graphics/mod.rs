use macroquad::prelude::Texture2D;
use macroquad::prelude::WHITE;

mod animated_texture;
mod three_way_texture;

pub fn draw(texture: Texture2D, x: f32, y: f32) {
    macroquad::prelude::draw_texture(texture, x, y, WHITE);
}

pub fn draw_scale(texture: Texture2D, x: f32, y: f32, scale: f32) {
    let dest_size = Some(macroquad::prelude::vec2(texture.width() * scale, texture.height() * scale));
    macroquad::prelude::draw_texture_ex(texture, x * scale, y * scale, WHITE, macroquad::prelude::DrawTextureParams {
        dest_size,
        ..Default::default()
    });
}