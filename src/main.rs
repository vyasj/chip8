use muda::{Menu, MenuEvent, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{ElementState, Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::platform::unix::WindowExtUnix as _;
use tao::window::{WindowBuilder};

use chip8::cpu::{Cpu, Instruction};

use std::env;
use std::sync::Arc;

fn main() -> Result<(), Error> {
    println!("Hello, CHIP-8!");
    let filename: String = env::args()
        .nth(1)
        .expect("Expected a single command line argument");

    println!("Initializing CPU...");
    let mut cpu = Cpu::init();

    println!("Loading rom...");
    cpu.load_rom(&filename);
    // cpu.print_ram();

    println!("Rendering display window...");
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new((cpu.width as f64)*5.0, (cpu.height as f64)*5.0);
        let window = WindowBuilder::new()
            .with_title("CHIP-8 shenanigans")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();
        Arc::new(window)
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(cpu.width as u32, cpu.height as u32, surface_texture)?
    };

    let menu = Menu::new();

    let file_menu = Submenu::new("File", true);
    menu.append(&file_menu).unwrap();
    file_menu
        .append(&muda::MenuItem::with_id("quit", "Quit", true, None))
        .unwrap();
    menu.init_for_gtk_window(window.gtk_window(), window.default_vbox())
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    cpu.dump_state();
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::KeyboardInput { event: inner, .. } => {
                    let key = inner.physical_key;
                    if key == KeyCode::Escape {
                        cpu.dump_state();
                        *control_flow = ControlFlow::Exit;
                    } else {
                        let keycode = match key {
                            KeyCode::Digit1 => 0x1,
                            KeyCode::Digit2 => 0x2,
                            KeyCode::Digit3 => 0x3,
                            KeyCode::Digit4 => 0xC,
                            KeyCode::KeyQ => 0x4,
                            KeyCode::KeyW => 0x5,
                            KeyCode::KeyE => 0x6,
                            KeyCode::KeyR => 0xD,
                            KeyCode::KeyA => 0x7,
                            KeyCode::KeyS => 0x8,
                            KeyCode::KeyD => 0x9,
                            KeyCode::KeyF => 0xE,
                            KeyCode::KeyZ => 0xA,
                            KeyCode::KeyX => 0x0,
                            KeyCode::KeyC => 0xB,
                            KeyCode::KeyV => 0xF,
                            _ => todo!(),
                        };
                        let state = match inner.state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                            _ => unreachable!(),
                        };
                        cpu.kp[keycode] = state;
                        *control_flow = ControlFlow::Poll;
                    }
                }

                WindowEvent::Resized(size) => {
                    if pixels.resize_surface(size.width, size.height).is_err() {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {
                    // println!("{:?}", event);
                }
            }

            Event::MainEventsCleared => {
                if cpu.pc as usize >= cpu.ram.len() {
                    println!("reached end of ram.");
                    cpu.dump_state();
                    std::process::exit(0x0100);
                }

                let bytes = cpu.fetch();

                if bytes == 0 {
                    println!("hit raw 0x0000");
                    cpu.dump_state();
                    std::process::exit(0x0100);
                }

                let instruction = Instruction::decode(bytes).unwrap();
                // println!("{}", instruction.to_string());
                let result = cpu.execute(instruction);

                if result.is_some() {
                    match result.unwrap() {
                        1 => window.request_redraw(),
                        2 => *control_flow = ControlFlow::Wait,
                        _ => {
                            println!("unknown return value from cpu.execute(), aborting...");
                            cpu.dump_state();
                            std::process::exit(0x0100);
                        }
                    };
                }

                if cpu.dt > 0 {
                    cpu.dt -= 1;
                }

                if cpu.st > 0 {
                    cpu.st -= 1;
                }

                *control_flow = ControlFlow::Poll;
            }

            Event::RedrawRequested(_) => {
                cpu.draw(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    println!("{}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {
                if let Ok(event) = MenuEvent::receiver().try_recv() {
                    if event.id.0 == "quit" {
                        cpu.dump_state();
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
        }
    });
}

