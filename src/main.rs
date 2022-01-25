use std::net::TcpStream;
use std::io::prelude::*;

fn main() {

    let screen = Screen {
        domain: String::from("skarm.ludd.ltu.se"), 
        port: String::from("1337"), 
        area: Area {x: 200, y: 100, height: 100, width: 100}
    };

    let image = ImageBuilder::new(String::from("test.png"));

    screen.write_graphic(&image);
}

#[derive(Default)]
#[derive(Clone)]
struct Pixel {
    coordinates: Coordinate,
    red: u8,
    blue: u8,
    green: u8,
    alpha: u8
}


#[derive(Default)]
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}



struct ImageBuilder;

impl ImageBuilder {
    fn new(path: String) -> image::DynamicImage {
        let image = image::open(path).unwrap();
       
        return image;
    }
}


impl Graphic for image::DynamicImage {
    fn as_pixels(&self) -> Box<Vec<Pixel>> {
        let raw = self.as_rgba8().unwrap();

        let width = raw.dimensions().0;
        let height = raw.dimensions().1;

        let total_pxl_count = width*height;

        let mut grid = vec![Pixel::default(); total_pxl_count as usize];
        let mut index: u32 = 0;

        for pi in 0..=height*width {

            

         
            let y = f32::floor((index/height) as f32) as u32;
            let x = (index%height)/4;

            let p = raw.get_pixel(x, y);


            let pixel = Pixel {
                red: p[0],
                green: p[1],
                blue: p[2],
                alpha: p[3],
                coordinates: Coordinate {x: x as usize, y: y as usize}
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
        if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", self.domain, self.port)) {
            let grid = *graphic.as_pixels();

            let mut output:String = String::from("");

            for pxl in grid as Vec<Pixel>{

                let mut pixel = pxl.clone();
                pixel.coordinates.x = pixel.coordinates.x + self.area.x;
                pixel.coordinates.y = pixel.coordinates.y + self.area.y;

                let command = pixel.as_command();
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

#[derive(Default)]
struct Area {
    x: usize,
    y: usize,
    width: u16,
    height: u16
}

trait Graphic {
    fn as_pixels(&self) -> Box<Vec<Pixel>>;
}

trait Hex {
    fn hex_fmt(&self) -> String;
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
