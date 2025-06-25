use glium::{glutin, Surface};

fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut frame = display.draw();
    let mut color = 0.0;
    let mut green = 0.0;
    let mut red = 0.0;
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut frame = display.draw();
                    color += 0.02;
                    if color > 1.0 {
                        color = 0.0;
                        green += 0.1;
                    }
                    if green > 1.0 {
                        green = 0.0;
                        red += 0.1;
                    }
                    if red > 1.0 {
                        red = 0.0;
                    }

                    frame.clear_color(red, green, color, 1.0);
                    println!("Color {color}");

                    frame.finish().unwrap();
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    });
}
