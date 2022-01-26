
    #[derive(Default)]
    #[derive(Clone)]
    pub struct Pixel {
        pub coordinates: Coordinate,
        pub red: u8,
        pub blue: u8,
        pub green: u8,
        pub alpha: u8
    }


    #[derive(Default)]
    #[derive(Clone)]
    pub struct Coordinate {
        pub x: usize,
        pub y: usize
    }

