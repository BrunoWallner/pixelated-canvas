mod engine;
use engine::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut window = Window::new([800, 800], "Test");

    let mut event_pump = window.event_pump();

    //game loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
    }
}