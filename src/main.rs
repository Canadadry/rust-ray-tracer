mod math;

#[derive(Copy, Clone)]
struct Pixel(u8,u8,u8,u8);

impl Pixel {
    pub fn red() -> Pixel { Pixel(255,0,0,255)}
}

fn main() {
    // let buffer = [Pixel::red();256];
    // lodepng::encode32_file("out.png", &buffer, 16,16);
}