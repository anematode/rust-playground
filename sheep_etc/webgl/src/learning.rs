use glium::{Surface, glutin, implement_vertex, uniform};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn load_image<'a>(bytes: &[u8]) -> glium::texture::RawImage2d<'a, u8> {
    let image = image::load(
        Cursor::new(bytes),
        image::ImageFormat::Png,
    ).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

pub fn main() {
    let vertex_shader_src = String::from_utf8_lossy(include_bytes!("./learning/vertex.vert")).into_owned();
    let fragment_shader_src = String::from_utf8_lossy(include_bytes!("./learning/fragment.frag")).into_owned();

    let image = load_image(&include_bytes!("./learning/texture.png")[..]);

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    ).unwrap();

    // Triangle vertices
    let vertex1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // A "dummy marker" because we only have one triangle so that WebGL knows what triangles
    // can be made from these vertices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.5, 1.0, 1.0);
        target.draw(
            &vertex_buffer,
            &indices,
            &program,
            &uniform! {
                matrix: [
                    [ t.cos(), t.sin(), 0.0, 0.0],
                    [-t.sin(), t.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ],
                tex: &texture,
            },
            &Default::default(),
        ).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        if let glutin::event::Event::WindowEvent { event, .. } = ev {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            return;
        }
    });
}
