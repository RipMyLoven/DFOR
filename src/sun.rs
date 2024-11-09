pub struct Sun {
    pub radius: i32,
    pub x: i32,
    pub y: i32,
}

impl Sun {
    pub fn new(radius: i32, x: i32, y: i32) -> Sun {
        Sun { radius, x, y }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for dy in -self.radius..=self.radius {
            for dx in -self.radius..=self.radius {
                if dx * dx + dy * dy <= self.radius * self.radius {
                    let px = self.x + dx;
                    let py = self.y + dy;

                    if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                        let index = (py as usize) * width + (px as usize);
                        buffer[index] = 0xFFFF00;
                    }
                }
            }
        }
    }
}
