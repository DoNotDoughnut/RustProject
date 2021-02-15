use macroquad::prelude::Vec2;

use crate::character::Character;
use crate::io::input;

pub struct Player {

    pos: Vec2,

    rot: f32,

    pub character: &'static Character,

}

impl Player {

    pub fn collision_check(&mut self, level: &super::level::Level) {

    }

}

impl crate::Entity for Player {

    fn update(&mut self, delta: f32) { // Multiplying speed by delta makes the speed constant across all framerates
        if input::is_control_down(input::Control::Up) {
            self.pos.y -= self.character.speed * delta;
            if self.pos.y < 0.0 {
                self.pos.y = 0.0;
            }
        }
        if input::is_control_down(input::Control::Down) {
            self.pos.y += self.character.speed * delta;
            if self.pos.y + self.character.hitbox.y > crate::HEIGHT as f32 { // Check if the hitbox of the player exceeds the bounds of the window
                self.pos.y = (crate::HEIGHT as f32) - self.character.hitbox.y;
            }
        }
        if input::is_control_down(input::Control::Left) {
            self.pos.x -= self.character.speed * delta;
            if self.pos.x < 0.0 {
                self.pos.x = 0.0;
            }
        }
        if input::is_control_down(input::Control::Right) {
            self.pos.x += self.character.speed * delta;
            if self.pos.x + self.character.hitbox.x > crate::WIDTH as _ {
                self.pos.x = (crate::WIDTH as f32) - self.character.hitbox.x;
            }
        }
        

        // let pos = (self.pos.x + self.texture.width() / 2.0, self.pos.y + self.texture.height() / 2.0);
        // let mouse = macroquad::prelude::mouse_position();
        // let x = mouse.0 - pos.0;
        // let y = mouse.1 - pos.1;
        // let rotation = if x > 0.0 {
        //     x.atan2(-y)
        // } else {
        //     (-x).atan2(y)
        // };
        //.asin() / y.acos();
        //macroquad::prelude::info!("{}", rotation);
        self.rot += 0.05;

        // let mut socket = crate::net::SOCKET.lock();

        // let data = nanoserde::SerBin::serialize_bin(&crate::net::PlayerData {
        //     x: self.pos.x,
        //     y: self.pos.y,
        // });
        
        // socket.send(laminar::Packet::unreliable_sequenced(*crate::net::SERVER, data, None)).unwrap();

        // socket.manual_poll(std::time::Instant::now());
         
    }

    fn render(&self) {
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