use glium::{Surface, glutin, implement_vertex, uniform};
use std::io::Cursor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
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
    let wb = glutin::window::WindowBuilder::new().with_title("b en t");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let program = glium::Program::from_source(
        &display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    ).unwrap();

    let vertices = vec![
        Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5, 0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices: Vec<u16> = vec![
        0, 1, 3,
        1, 2, 3,
    ];
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices
    ).unwrap();

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.005;

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.5, 1.0, 1.0), 1.0);
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                matrix: [
                    [ t.cos(), 0.0, t.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-t.sin(), 0.0, t.cos(), 0.0],
                    [0.0, 0.0, 2.0, 1.0f32],
                ],
                perspective: perspective,
                tex: &texture,
            },
            &params,
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
