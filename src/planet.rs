use image::{GenericImageView, imageops::FilterType};

pub struct Planet {
    pub radius: i32,        
    pub orbit_radius: i32,  
    pub angle: f32, 
    pub speed: f32,        
    pub x: i32,             
    pub y: i32,             
    pub texture: Option<Vec<u8>>,
    pub texture_width: usize,    
    pub texture_height: usize,   
}

impl Planet {
    pub fn new(radius: i32, orbit_radius: i32, angle: f32, speed:f32) -> Planet {
        Planet {
            radius,
            orbit_radius,
            angle,
            speed,
            x: 0,  
            y: 0,
            texture: None,
            texture_height: 0,
            texture_width: 0,
        }
    }

    pub fn load_texture(&mut self, image_path: &str) -> Result<(), String> {
        let img = match image::open(image_path) {
            Ok(img) => img,
            Err(_) => return Err("Failed to load image".to_string()),
        };
    
        let (width, height) = img.dimensions();
        let scale_factor = self.radius as f32 * 2.0 / width as f32;
        let new_width = (width as f32 * scale_factor) as u32;
        let new_height = (height as f32 * scale_factor) as u32;
        let img = img.resize(new_width, new_height, FilterType::Lanczos3);
    
        self.texture_width = img.width() as usize;
        self.texture_height = img.height() as usize;
    
        let img = img.to_rgb8();
        self.texture = Some(img.into_raw());
    
        Ok(())
    }
    

    pub fn update_position(&mut self, center_x: i32, center_y: i32, delta_time: f32) {
        self.angle += self.speed * delta_time;
        if self.angle > 360.0 {
            self.angle -= 360.0;
        }
    
        self.x = center_x + (self.orbit_radius as f32 * self.angle.to_radians().cos()) as i32;
        self.y = center_y + (self.orbit_radius as f32 * self.angle.to_radians().sin()) as i32;
    }
    
    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        if let Some(texture) = &self.texture {
            for dy in 0..self.texture_height {
                for dx in 0..self.texture_width {
                    let px = self.x + dx as i32 - (self.texture_width / 2) as i32;
                    let py = self.y + dy as i32 - (self.texture_height / 2) as i32;
    
                    if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                        let pixel_index = (dy * self.texture_width + dx) * 3;
                        if pixel_index + 2 < texture.len() {
                            let r = texture[pixel_index] as u32;
                            let g = texture[pixel_index + 1] as u32;
                            let b = texture[pixel_index + 2] as u32;
    
                            let color = (r << 16) | (g << 8) | b; 
    
                            let index = (py as usize) * width + (px as usize);
                            if index < buffer.len() {
                                buffer[index] = color;
                            }
                        }
                    }
                }
            }
        } else {
            for dy in -self.radius..=self.radius {
                for dx in -self.radius..=self.radius {
                    if dx * dx + dy * dy <= self.radius * self.radius {
                        let px = self.x + dx;
                        let py = self.y + dy;
    
                        if px >= 0 && py >= 0 && px < width as i32 && py < height as i32 {
                            let index = (py as usize) * width + (px as usize);
                            if index < buffer.len() {
                                buffer[index] = 0xFF0000;
                            }
                        }
                    }
                }
            }
        }
    }
}
