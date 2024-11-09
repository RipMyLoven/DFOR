mod planet;
mod sun;
use minifb::{Window, WindowOptions};
use planet::Planet;
use sun::Sun;

fn main() {
    let width = 800;
    let height = 600;

    let mut window = Window::new(
        "Solar System",
        width,
        height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Cannot create planets!!!");

    let mut buffer: Vec<u32> = vec![0; width * height]; 

    let sun = Sun::new(50, width as i32 / 2, height as i32 / 2);

    let mut planet1 = Planet::new(10, 150, 0.0); 
    let mut planet2 = Planet::new(20, 250, 90.0);
    let mut planet3 = Planet::new(30, 350, 180.0);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        
        buffer.fill(0x000000);

        sun.draw(&mut buffer, width, height);

        planet1.update_position(width as i32 / 2, height as i32 / 2);
        planet1.draw(&mut buffer, width, height);

        planet2.update_position(width as i32 / 2, height as i32 / 2);
        planet2.draw(&mut buffer, width, height);

        planet3.update_position(width as i32 / 2, height as i32 / 2);
        planet3.draw(&mut buffer, width, height);

        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
