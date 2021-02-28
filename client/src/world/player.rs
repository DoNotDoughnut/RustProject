use macroquad::prelude::Vec2;
use macroquad::prelude::get_frame_time;
use macroquad::prelude::scene::RefMut;
use crate::character::Character;
use crate::io::input;

pub struct Player {

    pub pos: Vec2,

    rot: f32,

    pub character: &'static Character,

}

impl Player {

    pub fn collision_check(&mut self, level: &super::level::Level) {

    }

    pub fn render(&self) {
        macroquad::texture::draw_texture_ex(self.character.texture, self.pos.x * crate::SCALE, self.pos.y * crate::SCALE, macroquad::prelude::WHITE, macroquad::prelude::DrawTextureParams {
            dest_size: Some(macroquad::prelude::vec2(self.character.texture.width() * crate::SCALE, self.character.texture.height() * crate::SCALE)),
            rotation: self.rot,
            ..Default::default()
        }); // Draw the player image
    }

}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: Vec2::new(30.0, 30.0),
            rot: 0.0,
            character: &crate::character::NAREG,
        }
    }
}

impl macroquad::prelude::scene::Node for Player {

    fn update(mut node: RefMut<Self>) {

        node.rot += 0.05;

        let delta = get_frame_time();
        if input::is_control_down(input::Control::Up) {
            node.pos.y -= node.character.speed * delta;
            if node.pos.y < 0.0 {
                node.pos.y = 0.0;
            }
        }
        if input::is_control_down(input::Control::Down) {
            node.pos.y += node.character.speed * delta;
            if node.pos.y + node.character.hitbox.y > crate::HEIGHT as f32 { // Check if the hitbox of the player exceeds the bounds of the window
                node.pos.y = (crate::HEIGHT as f32) - node.character.hitbox.y;
            }
        }
        if input::is_control_down(input::Control::Left) {
            node.pos.x -= node.character.speed * delta;
            if node.pos.x < 0.0 {
                node.pos.x = 0.0;
            }
        }
        if input::is_control_down(input::Control::Right) {
            node.pos.x += node.character.speed * delta;
            if node.pos.x + node.character.hitbox.x > crate::WIDTH as _ {
                node.pos.x = (crate::WIDTH as f32) - node.character.hitbox.x;
            }
        }
    }

    fn draw(node: RefMut<Self>) {
        node.render();
    }
}