use crate::base::{Pixel, Area};

pub trait Graphic {
    fn as_pixels(&self, area: &Area) -> Box<Vec<Pixel>>;
}
        
pub trait Hex {
    fn hex_fmt(&self) -> String;
}




