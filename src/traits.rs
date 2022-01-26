use crate::base::Pixel;

pub trait Graphic {
    fn as_pixels(&self) -> Box<Vec<Pixel>>;
}
        
pub trait Hex {
    fn hex_fmt(&self) -> String;
}



