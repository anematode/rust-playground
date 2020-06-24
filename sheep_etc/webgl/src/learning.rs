use glium::{Surface, glutin, implement_vertex};
use std::fs;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub fn main() {
    let vertex_shader_src = fs::read_to_string("./src/learning/vertex.vert")
        .expect("Getting vertex shader was problematic");
    let fragment_shader_src = fs::read_to_string("./src/learning/fragment.frag")
        .expect("Getting fragment shader was problematic");

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap();

    // Triangle vertices
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // A "dummy marker" because we only have one triangle so that WebGL knows what triangles
    // can be made from these vertices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.5, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
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
