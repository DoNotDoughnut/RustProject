use macroquad::{
    experimental::{
        scene::{self, RefMut},
    },
    prelude::*,
};
use crate::world::player::Player;

pub struct RemotePlayer {
    pub network_id: String,
    player: Player,

    // dead: bool,
    pos_delta: Vec2,
    last_move_time: f64,
}

impl RemotePlayer {
    pub fn new(network_id: String) -> RemotePlayer {
        let pos = vec2(100., 105.);

        RemotePlayer {
            player: Player::default(),
            network_id,
            pos_delta: vec2(0.0, 0.0),
            last_move_time: 0.0,
            // dead: false,
        }
    }

    // pub fn pick_weapon(&mut self) {
    //     self.fish.pick_weapon()
    // }

    // pub fn disarm(&mut self) {
    //     self.fish.disarm()
    // }

    // pub fn armed(&self) -> bool {
    //     self.fish.armed()
    // }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.last_move_time = get_time();
        self.pos_delta = pos - self.player.pos;
        self.player.pos = pos;
    }

    pub fn set_facing(&mut self, facing: bool) {
        // self.fish.set_facing(facing);
    }

    // pub fn set_dead(&mut self, dead: bool) {
    //     self.dead = dead;
    // }

    pub fn pos(&self) -> Vec2 {
        self.player.pos
    }
}

impl scene::Node for RemotePlayer {
    fn draw(mut node: RefMut<Self>) {
        draw_text_ex(
            &node.network_id[0..5],
            node.player.pos.x - 1.,
            node.player.pos.y - 1.,
            TextParams {
                font_size: 50,
                font_scale: 0.25,
                ..Default::default()
            },
        );

        node.player.render();
    }

    fn update(mut node: RefMut<Self>) {
        // if node.dead {
        //     let resources = storage::get::<Resources>().unwrap();
        //     let on_ground = resources
        //         .collision_world
        //         .collide_check(node.fish.collider, node.fish.pos() + vec2(0., 1.));

        //     if on_ground {
        //         node.fish.set_animation(3);
        //     } else {
        //         node.fish.set_animation(2);
        //     }
        // } else if get_time() - node.last_move_time > 0.2 || node.pos_delta.length() < 0.01 {
        //     node.fish.set_animation(0);
        // } else {
        //     node.fish.set_animation(1);
        // }
    }
}