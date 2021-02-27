use glium::glutin;
use glium::Surface;
use glium::implement_vertex;
use glium::uniform;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

/*
fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //shaders
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 attr;

        uniform mat4 matrix;

        void main() {
            attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
    #version 140

    in vec2 attr;
    out vec4 color;

    void main() {
        color = vec4(attr, attr[0] * attr[1], 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut animation: f32 = -0.5;

    event_loop.run(move |ev, _, control_flow| {
        animation +=0.0001;
        if animation > 0.5 { animation = -0.5; }
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0], //rotation
                [0.0, 1.0, 0.0, 0.0], //scale
                [0.0, 0.0, 0.5, 0.0], //IDK
                [animation, animation, 0.0, 1.0f32], //position
            ]
        };

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();

        target.finish().unwrap();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
*/

pub struct Window {
    display: glium::Display,
    framebuffer: Vec<Pixel>
}

pub struct event_loop;
impl event_loop {
    pub fn new() -> glutin::event_loop::EventLoop<()> {
        glutin::event_loop::EventLoop::new()
    }
}

impl Window {
    pub fn new(ev: glutin::event_loop::EventLoop<()>) -> Self {
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();

        Window{ display: glium::Display::new(wb, cb, &ev).unwrap(), framebuffer: vec!() }
    }

    pub fn get_display(&self) -> &glium::Display {
        &self.display
    }

    pub fn draw(&mut self, pixel: Pixel) {
        self.framebuffer.push(pixel);
    }

    pub fn update(&self, display: &glium::Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Pixel {
    pub position: [f32; 2],
    pub color: [f32; 4]
}