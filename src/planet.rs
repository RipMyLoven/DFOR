pub struct Planet {
    pub radius: i32,        
    pub orbit_radius: i32,  
    pub angle: f32,         
    pub x: i32,             
    pub y: i32,             
}

impl Planet {
    pub fn new(radius: i32, orbit_radius: i32, angle: f32) -> Planet {
        Planet {
            radius,
            orbit_radius,
            angle,
            x: 0,  
            y: 0,  
        }
    }

    pub fn update_position(&mut self, center_x: i32, center_y: i32) {
        self.angle += 0.01; 
        if self.angle > 360.0 {
            self.angle -= 360.0;
        }

        self.x = center_x + (self.orbit_radius as f32 * self.angle.to_radians().cos()) as i32;
        self.y = center_y + (self.orbit_radius as f32 * self.angle.to_radians().sin()) as i32;
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for dy in -self.radius..=self.radius {
            for dx in -self.radius..=self.radius {
                if dx * dx + dy * dy <= self.radius * self.radius {
                    let px = self.x + dx;
                    let py = self.y + dy;

                    if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                        let index = (py as usize) * width + (px as usize);
                        buffer[index] = 0xFF0000;
                    }
                }
            }
        }
    }
}
