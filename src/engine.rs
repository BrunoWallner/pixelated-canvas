
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;

pub struct Window {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<sdl2::video::Window>
}
impl Window {
    pub fn new(size: [u32; 2], title: &str) -> Self {
        let sdl_c = sdl2::init().unwrap();
        let video_s= sdl_c.video().unwrap();

        let win = video_s.window(title, size[0], size[1])
        .position_centered()
        .build()
        .unwrap();

        let can = win.into_canvas().build().unwrap();

        Window{ sdl_context: sdl_c, canvas: can }
    }

    pub fn event_pump(&self) -> sdl2::EventPump {
        self.sdl_context.event_pump().unwrap()
    }
}