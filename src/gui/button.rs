use macroquad::prelude::Color;
use macroquad::prelude::WHITE;

pub struct Button {

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub text: Option<String>, // Options are like null, can either store a variable or not
    button_params: ButtonParams, // Extra variables so there doesnt need to be math done every render

}

impl Button {

    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, text: Option<&str>) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            text: text.map(|text| text.to_owned()),
            button_params: ButtonParams {
                thickness: (height / 8.0).ceil(),
                text_x: x + 10.0,
                text_y: y + height / 2.0,
                font_size: height / 2.0,
            },
        }
    }

    pub fn render(&self, selected: bool) {
        macroquad::prelude::draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        if selected {
            // If selected, render a white border around the button
            macroquad::prelude::draw_rectangle_lines(self.x, self.y, self.width, self.height, self.button_params.thickness, WHITE);
        }
        if let Some(text) = self.text.as_ref() { // Check if the button has text
            // Draw the text of the button
            macroquad::prelude::draw_text(&text, self.button_params.text_x, self.button_params.text_y, self.button_params.font_size, WHITE);
        }
    }

}

struct ButtonParams {
    thickness: f32,
    text_x: f32,
    text_y: f32,
    font_size: f32,
}