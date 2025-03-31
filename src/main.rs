use indicatif::{ProgressBar, ProgressStyle};
use rtrs::color::gamma_correction;
use rtrs::{camera::CameraBuilder, image::ImageInfo, scenes::test_scene, vec3::Point3};
use std::{fs::File, io::BufWriter};

use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod winit_app;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let file = File::create("img.ppm").unwrap();
    let mut file = BufWriter::new(file);

    let world = test_scene();
    let image_info = ImageInfo::from_aspect(144, 16.0 / 9.0);

    let mut cam = CameraBuilder::new(
        &image_info,
        &Point3(13.0, 2.0, 3.0),
        &Point3(0.0, 0.0, 0.0),
    )
    .samples_per_pixel(4)
    .fov(20.0)
    .max_depth(50)
    .defocus_angle(0.6)
    .focus_dist(10.0)
    .build();

    let render_bar = {
        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} ({per_sec})",
        )
        .unwrap()
        .progress_chars("#>.");
        let bar = ProgressBar::new(image_info.image_height as u64);
        bar.set_style(style);
        bar
    };

    let image = cam.render(&world, Some(&render_bar));
    render_bar.finish();

    let _ = cam.write_image(&mut file, &image);

    let event_loop = EventLoop::new().unwrap();

    let mut app = winit_app::WinitAppBuilder::with_init(
        |elwt| {
            let window = {
                let window = elwt.create_window(Window::default_attributes());
                Rc::new(window.unwrap())
            };
            let context = softbuffer::Context::new(window.clone()).unwrap();

            (window, context)
        },
        |_elwt, (window, context)| softbuffer::Surface::new(context, window.clone()).unwrap(),
    )
    .with_event_handler(|(window, _context), surface, event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let Some(surface) = surface else {
                    eprintln!("RedrawRequested fired before Resumed or after Suspended");
                    return;
                };
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                cam.initialize(ImageInfo::from_dim(width, height), 1, 20.0);
                let image = cam.render(&world, None);


                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) {
                    let y = index / width;
                    let x = index % width;
                    if y >= cam.image_info.image_height || x >= cam.image_info.image_width {
                        continue;
                    }
                    let (red, green, blue) = gamma_correction(&image[(cam.image_info.image_width*y + x) as usize]);

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    });

    event_loop.run_app(&mut app).unwrap();
}
