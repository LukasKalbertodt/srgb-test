#[macro_use]
extern crate glium;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let gl_window = glutin::ContextBuilder::new()
        .with_srgb(false)
        .build_windowed(wb, &events_loop)
        .unwrap();


    let gl_window = unsafe { gl_window.make_current().unwrap() };
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    let mut out: gl::types::GLint = 0;
    unsafe {
        gl::GetFramebufferAttachmentParameteriv(
            gl::DRAW_FRAMEBUFFER,
            gl::BACK_LEFT,
            gl::FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING,
            &mut out as *mut _,
        );
    }
    print!("What OpenGL thinks: ");
    if out == gl::LINEAR as i32 {
        print!("linear");
    } else if out == gl::SRGB as i32 {
        print!("srgb");
    }
    println!(" ({})", out);


    println!("What glutin thinks: srgb = {}", gl_window.get_pixel_format().srgb);

    // use crate::glium::glutin::os::ContextTraitExt;
    // println!("{:?}", unsafe { gl_window.get_egl_display() });


    let display = glium::Display::from_gl_window(gl_window).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let shape = vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
        Vertex { position: [ 1.0,  1.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(vec3(0.5), 1.0);
        }
    "#;

    let program = glium::Program::new(
        &display,
        glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: vertex_shader_src,
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: fragment_shader_src,
            transform_feedback_varyings: None,
            outputs_srgb: false,
            uses_point_size: false,
        }
    ).unwrap();

    loop {
        let mut target = display.draw();

        target.draw(&vertex_buffer, &indices, &program, &uniform!{}, &Default::default()).unwrap();
        target.finish().unwrap();

        // We don't need high FPS
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
