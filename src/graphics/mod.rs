use macroquad::prelude::Texture2D;

mod animated_texture;
mod three_way_texture;

pub fn draw(texture: Texture2D, x: f32, y: f32) {
    macroquad::prelude::draw_texture(texture, x, y, macroquad::prelude::WHITE);
}