use std::fmt::{self, Formatter, Display};

struct City{
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 'write!' will write the formatted string into a buffer;
        write!(f, "{}: {:.3}.{} {:.3}.{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

// #[derive(Debug)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let red = self.red;
        let green = self.green;
        let blue = self.blue;

        write!(f, "RGB ({red}, {green}, {blue}), 0x{:02X}{:02X}{:02X}", red, green, blue)
    }
}

fn main()
{
    for city in [
        City { name: "Dublin",   lat: 53.4337, lon: -6.259722},
        City { name: "Beijing",  lat: 10.1002, lon: -6.3321},
        City { name: "Vancouver",lat: 49.25,   lon:-123.1},
    ].iter() {
        println!("{}", *city);
    }

    let a = 128;
    let mov = 4;
    println!("{} = 0x{:X}{:X}", a, a, a);

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0,   green: 3,   blue: 254},
        Color { red: 0,   green: 0,   blue: 0},
    ].iter() {
        println!("{}", *color);
    }
}