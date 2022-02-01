pub mod base;
pub mod traits;

use std::net::TcpStream;
use std::io::prelude::*;
use crate::traits::*;
use crate::base::*;
use std::{thread, time};


fn main() {

    

    //let refresh_time = time::Duration::from_millis(20);


    

    let screen = Screen {
        domain: String::from("skarm.ludd.ltu.se"), 
        port: String::from("1337"), 
        area: Area {x: 680, y: 10}
    };

    let image = ImageBuilder::new(String::from("hakan.png"));
    let grid = image.as_pixels(&screen.area);

    loop {
        screen.write_grid(&grid);
        //thread::sleep(refresh_time);
    }
    
}





struct ImageBuilder;

impl ImageBuilder {
    fn new(path: String) -> image::DynamicImage {
        let image = image::open(path).unwrap();
       
        return image;
    }
}


impl traits::Graphic for image::DynamicImage {
    fn as_pixels(&self, area: &Area) -> Box<Vec<base::Pixel>> {
        let raw = self.as_rgba8().unwrap();

        let width = (raw.dimensions().0-1) as usize;
        let height = (raw.dimensions().1-1) as usize;

        let total_pxl_count = width*height;

        let mut grid = vec![Pixel::default(); total_pxl_count as usize];
        let mut index: usize = 0;

        for pix in 0..=height*width {

            let y:usize = pix/width;
            let x = pix%width;

            let p = raw.get_pixel(x as u32, y as u32);


            let pixel = Pixel {
                red: p[0],
                green: p[1],
                blue: p[2],
                alpha: p[3],
                coordinates: Coordinate {x: x+area.x as usize, y: y+area.y as usize}
            };

            grid.push(pixel);

            index = index + 1;

        }

        let out: Box<Vec<Pixel>> = Box::new(grid);


        return out;
    }
}

struct Screen {
    area: Area,
    domain: String,
    port: String
}

impl Screen {
    fn write_graphic(&self, graphic: &dyn Graphic) {
        self.write_grid(&graphic.as_pixels(&self.area));
    }

    fn write_grid(&self, grid: &Box<Vec<Pixel>>) {
        if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", self.domain, self.port)) {
            let grid_raw = &*(*grid);

            let mut output:String = String::from("");

            for pxl in grid_raw as &Vec<Pixel>{

            
                let command = pxl.as_command();
                output = output + &command;
                
            }

            let bytes = output.as_bytes();
            let _res = stream.write(&bytes);
            let push = stream.flush();

            //println!("{}", output);

            match push {
                Err(why) => {print!("its fucked {}", why)},
                Ok(_code) => {print!("Working")}
            }
        }
    }
}

impl Pixel {
    fn as_command(&self) -> String {
        return format!("PX {} {} {}{}{}{} \n",
                self.coordinates.x, self.coordinates.y, self.red.hex_fmt(), self.green.hex_fmt(), self.blue.hex_fmt(), self.alpha.hex_fmt());
    }
}





impl Hex for u8 {
    fn hex_fmt(&self) -> String {
        let out = format!("{:x}", self);
        if String::len(&out) == 2 {
            return out;
        } else {
            return String::from("0") + &out;
        }
    }
}
