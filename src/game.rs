use crate::input::{Input, Control};

pub struct Game {

    pub input: Input,

    x: f64, // cube x and y
    y: f64,

}

impl Game {

    pub fn new() -> Self {
        Self {
            input: Input::new(),
            x: 200.0,
            y: 200.0,
        }
    }

    pub fn load(&mut self) {

    }
    
    pub fn update(&mut self) {
    
    }
    
    pub fn render(&self, ctx: &mut piston_window::Context, g: &mut piston_window::G2d) {
        piston_window::rectangle_from_to([1.0, 0.0, 0.0, 1.0], [self.x, self.y], [self.x + 50.0, self.y + 50.0], ctx.transform, g);
    }
    
    pub fn input(&mut self) {
        if self.input.is_down(Control::Up) {
            self.y -= 3.0;
        }
        if self.input.is_down(Control::Down) {
            self.y += 3.0;
        }
        if self.input.is_down(Control::Left) {
            self.x -= 3.0;
        }
        if self.input.is_down(Control::Right) {
            self.x += 3.0;
        }
    }
    
    pub fn quit(&mut self) {
    
    }

}

