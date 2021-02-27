mod engine;
use engine::*;

fn main() {
    let mut ev = event_loop::new();
    let mut window = Window::new(ev);
    let mut pixel = Pixel{ position: [0.25, 0.25], color: [1.0, 1.0, 1.0, 1.0] };

    //game loop
    loop {
        window.draw(pixel);
        window.update(window.get_display());
    }
}