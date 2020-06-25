use glium::{Surface, glutin, implement_vertex, uniform};
use std::{io::Cursor, f32::consts::PI, time::{Instant, Duration}};
use nalgebra::{Matrix4, Vector3};

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

    let start = Instant::now();
    event_loop.run(move |ev, _, control_flow| {
        let now = Instant::now();
        let next_frame_time = now + Duration::from_nanos(16_666_667);
        let total_elapsed = now.duration_since(start).as_secs_f32();

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();
        let model = Matrix4::from_euler_angles(
            PI / 6.0 * (total_elapsed * 2.0 * PI).sin(),
            total_elapsed * 2.0 * PI / 5.0,
            0.0,
        ).append_translation(&Vector3::new(0.0, 0.0, -2.0));
        let model_ref = model.as_ref();
        let perspective = Matrix4::new_perspective(
            width as f32 / height as f32,
            PI / 3.0,
            0.1,
            1024.0,
        );
        // https://github.com/glium/glium/issues/1681#issuecomment-375083916
        let perspective_ref = perspective.as_ref();
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // Uncomment for one-sided triangles:
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        target.clear_color_and_depth((0.0, 0.5, 1.0, 1.0), 1.0);
        target.draw(
            &vertex_buffer,
            &index_buffer,
            &program,
            &uniform! {
                matrix: *model_ref,
                perspective: *perspective_ref,
                tex: &texture,
            },
            &params,
        ).unwrap();
        target.finish().unwrap();


        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        if let glutin::event::Event::WindowEvent { event, .. } = ev {
            if event == glutin::event::WindowEvent::CloseRequested {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            return;
        }
    });
}
