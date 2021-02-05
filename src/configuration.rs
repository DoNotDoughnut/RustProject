use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub struct Configuration {

    pub name: String,
    pub window_width: u32,
    pub window_height: u32,

}

impl Default for Configuration {
    fn default() -> Self {
        use crate::SCALE;
        Self {
            name: crate::NAME.to_owned(),
            window_width: (crate::WIDTH*SCALE) as _,
            window_height: (crate::HEIGHT*SCALE) as _,
        }
    }
}