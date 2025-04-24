use muda::{Menu, MenuEvent, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent, ElementState};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::platform::unix::WindowExtUnix as _;
use tao::window::WindowBuilder;

use chip8::cpu::Cpu;

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

    println!("Rendering display window...");
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(cpu.width as f64, cpu.height as f64);
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

    let mut debug_mode = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Escape,
                            ..
                        },
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Digit1,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x1] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Digit2,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x2] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Digit3,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x3] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::Digit4,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xC] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyQ,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x4] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyW,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x5] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyE,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x6] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyR,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xD] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyA,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x7] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyS,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x8] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyD,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0x9] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyF,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xE] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyZ,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xA] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyX,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyC,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xB] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyV,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    cpu.kp[0xF] = true;
                    debug_mode = false;
                    *control_flow = ControlFlow::Poll;
                },

                WindowEvent::Resized(size) => {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("{}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                },

                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: KeyCode::KeyJ,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    if !debug_mode {
                        println!("entering debug mode. press any other key to continue.");
                    }
                    debug_mode = true;
                    *control_flow = ControlFlow::Wait;
                },

                _ => {
                    //
                }
            },

            Event::MainEventsCleared => {
                if cpu.pc as usize >= cpu.ram.len() {
                    println!("reached end of ram.");
                    return;
                }

                let instruction = cpu.fetch();
                let opcode = cpu.decode(instruction);
                let result = cpu.execute(opcode);

                if result.is_some() {
                    let val = result.unwrap();
                    if val == 1 {
                        // Request redraw from window
                        window.request_redraw();
                    } else if val == 2 {
                        // Wait for key press
                        *control_flow = ControlFlow::Wait;
                    }
                }

                if debug_mode {
                    println!("--------------------------------------------------");
                    println!("program counter: {}", cpu.pc);

                    println!("stack pointer: {}", cpu.sp);

                    println!("index register: {}", cpu.ir);

                    print!("stack: ");
                    for idx in 0..cpu.stack.len() {
                        print!("|{}", cpu.stack[idx]);
                    }
                    println!("|");

                    print!("v registers: ");
                    for idx in 0..cpu.vx.len() {
                        print!("|{}", cpu.vx[idx]);
                    }
                    println!("|");

                    println!("sound timer/delay timer: {}/{}", cpu.st, cpu.dt);

                    println!("processing opcode: {:#x}", instruction);

                    println!("--------------------------------------------------");
                }

                

                if cpu.dt > 0 {
                    cpu.dt -= 1;
                }

                if cpu.st > 0 {
                    cpu.st -= 1;
                }

                cpu.kp = vec![false; 16];
            },

            Event::RedrawRequested(_) => {
                cpu.draw(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    println!("{}", err);
                    *control_flow = ControlFlow::Exit;
                }
            },

            _ => {
                if let Ok(event) = MenuEvent::receiver().try_recv() {
                    if event.id.0 == "quit" {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fonts() {
        let memory = Cpu::init();

        let exp_result = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        assert_eq!(memory.ram[0x050..0x0A0], exp_result);
    }

    #[test]
    fn test_display() {
        let mut display = Cpu::init();

        let screen_copy: Vec<bool> = vec![false; 2048];

        display.screen[0..32].fill(true);

        let mut exp_result: Vec<bool> = vec![false; 2048];
        exp_result[0..32].fill(true);

        let result: Vec<bool> = display
            .screen
            .iter()
            .zip(screen_copy.iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        assert_eq!(exp_result, result);
    }

    
}