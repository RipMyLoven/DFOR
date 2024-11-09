mod planet;
mod sun;
use minifb::{Window, WindowOptions};
use planet::Planet;
use sun::Sun;
use std::time::Instant;

fn load_planet_texture(planet: &mut Planet, image_path: &str) {
    if let Err(e) = planet.load_texture(image_path) {
        eprintln!("Error loading texture: {}", e);
    }
}

fn main() {
    let width = 1000;
    let height = 1000;

    let mut window = Window::new(
        "Solar System",
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Cannot create window");

    let mut buffer: Vec<u32> = vec![0; width * height]; 

    let sun = Sun::new(50, width as i32 / 2, height as i32 / 2);

    let mut planets = vec![
        Planet::new(5, 60, 0.0, 0.02),  
        Planet::new(10, 80, 45.0, 0.015),  
        Planet::new(12, 100, 90.0, 0.01),  
        Planet::new(8, 130, 135.0, 0.008),  
        Planet::new(25, 200, 180.0, 0.005),  
        Planet::new(25, 300, 225.0, 0.003),  
        Planet::new(15, 375, 270.0, 0.002), 
        Planet::new(15, 450, 315.0, 0.001),  
        Planet::new(5, 475, 360.0, 0.0005), 
    ];

    let planet_textures = vec![
        "img/Mercury.png",
        "img/Venus.png",
        "img/Earth.png",
        "img/Mars.png",
        "img/Jupiter.png",
        "img/Saturn.png",
        "img/Uranus.png",
        "img/Neptune.png",
        "img/Pluto.png",
    ];

    for (planet, texture) in planets.iter_mut().zip(planet_textures.iter()) {
        load_planet_texture(planet, texture);
    }

    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        let delta_time = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();

        buffer.fill(0x000000);  

        
        sun.draw(&mut buffer, width, height);

        
        for planet in planets.iter_mut() {
            planet.update_position(width as i32 / 2, height as i32 / 2, delta_time);
            planet.draw(&mut buffer, width, height);
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
