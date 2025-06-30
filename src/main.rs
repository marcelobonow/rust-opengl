use glium::{glutin, implement_vertex, Surface};

use crate::shader_manager::load_shader;
mod shader_manager;

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");

    ///TODO: Trocar de simple window builder para glium::window::WindowBuilder que permite escolher a versão do opengl
    ///Habilitar vsync entre outras coisas
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    window.set_title("Voce caiu na pegadinha do mario games");

    // glutin::
    // let (_window, display) = glium::window

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, 0.45] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader = load_shader("vertex.glsl");
    let frag_shader = load_shader("fragment.glsl");

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), frag_shader.as_str(), None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();

    // draw the triangle here
    target.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
