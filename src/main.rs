use muda::{Menu, MenuEvent, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent, ElementState};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::platform::unix::WindowExtUnix as _;
use tao::window::WindowBuilder;

use chip8::disp::Display;
use chip8::mem::Memory;
use chip8::reg::Registers;

use std::env;
use std::sync::Arc;

fn main() -> Result<(), Error> {
    println!("Hello, CHIP-8!");
    let filename: String = env::args()
        .nth(1)
        .expect("Expected a single command line argument");

    println!("Initializing memory...");
    let mut memory: Memory = Memory::init();

    println!("Loading fonts...");
    memory.load_font();

    println!("Loading rom...");
    memory.load_rom(&filename);

    println!("Initializing registers...");
    let mut registers: Registers = Registers::init();

    println!("Initializing display cache...");
    let mut display = Display::init();

    println!("Rendering display window...");
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(display.width as f64, display.height as f64);
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
        Pixels::new(display.width as u32, display.height as u32, surface_texture)?
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
                    registers.kp[0x1] = true;
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
                    registers.kp[0x2] = true;
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
                    registers.kp[0x3] = true;
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
                    registers.kp[0xC] = true;
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
                    registers.kp[0x4] = true;
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
                    registers.kp[0x5] = true;
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
                    registers.kp[0x6] = true;
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
                    registers.kp[0xD] = true;
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
                    registers.kp[0x7] = true;
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
                    registers.kp[0x8] = true;
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
                    registers.kp[0x9] = true;
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
                    registers.kp[0xE] = true;
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
                    registers.kp[0xA] = true;
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
                    registers.kp[0] = true;
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
                    registers.kp[0xB] = true;
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
                    registers.kp[0xF] = true;
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
                if debug_mode {
                    println!("stack:");
                    for idx in 0..memory.stack.len() {
                        print!("|{}", memory.stack[idx]);
                    }
                    print!("|");
                    println!("\n");
                }

                let n1: u16 = memory.ram[registers.pc as usize] as u16;
                let n2: u16 = memory.ram[(registers.pc + 1) as usize] as u16;
                registers.pc += 2;

                if registers.pc as usize >= memory.ram.len() {
                    println!("reached end of ram.");
                    return;
                }

                let opcode: u16 = (n1 << 8) | n2;
                let digit1: u16 = (0xF000 & opcode) >> 12;
                let digit2: u16 = (0x0F00 & opcode) >> 8;
                let digit3: u16 = (0x00F0 & opcode) >> 4;
                let digit4: u16 = 0x000F & opcode;

                match (digit1, digit2, digit3, digit4) {
                    // CLS
                    (0, 0, 0xE, 0) => {
                        for x in &mut display.screen {
                            *x = false;
                        }
                    },
                    // RET
                    (0, 0, 0xE, 0xE) => {
                        registers.sp -= 1;
                        registers.pc = memory.stack[registers.sp as usize];
                    },
                    // SYS NNN
                    (0, _, _, _) => {
                        //println!("subroutine call: {:#x} is not implemented because it is a special subroutine call for the physical device.", opcode);
                    },
                    // JP NNN
                    (0x1, _, _, _) => {
                        registers.pc = (digit2 << 8) | (digit3 << 4) | digit4;
                    },
                    // CALL NNN
                    (0x2, _, _, _) => {
                        memory.stack[registers.sp as usize] = registers.pc;
                        registers.pc = (digit2 << 8) | (digit3 << 4) | digit4;
                        registers.sp += 1;
                    },
                    // SE Vx, kk
                    (0x3, _, _, _) => {
                        if registers.vx[digit2 as usize] == ((digit3 << 4) | digit4) as u8 {
                            registers.pc += 2;
                        }
                    },
                    // SNE Vx, kk
                    (0x4, _, _, _) => {
                        if registers.vx[digit2 as usize] != ((digit3 << 4) | digit4) as u8 {
                            registers.pc += 2;
                        }
                    },
                    // SE Vx, Vy
                    (0x5, _, _, 0) => {
                        if registers.vx[digit2 as usize] == registers.vx[digit3 as usize] {
                            registers.pc += 2;
                        }
                    },
                    // LD Vx, kk
                    (0x6, _, _, _) => {
                        registers.vx[digit2 as usize] = ((digit3 << 4) | digit4) as u8;
                    },
                    // ADD Vx, kk
                    (0x7, _, _, _) => {
                        let sum: u16 = (registers.vx[digit2 as usize] as u16) + ((digit3 << 4) | digit4);
                        registers.vx[digit2 as usize] = sum as u8;
                    },
                    // LD Vx, Vy
                    (0x8, _, _, 0) => {
                        registers.vx[digit2 as usize] = registers.vx[digit3 as usize];
                    },
                    // OR Vx, Vy
                    (0x8, _, _, 0x1) => {
                        registers.vx[digit2 as usize] |= registers.vx[digit3 as usize];
                    },
                    // AND Vx, Vy
                    (0x8, _, _, 0x2) => {
                        registers.vx[digit2 as usize] &= registers.vx[digit3 as usize];
                    },
                    // XOR Vx, Vy
                    (0x8, _, _, 0x3) => {
                        registers.vx[digit2 as usize] ^= registers.vx[digit3 as usize];
                    },
                    // ADD Vx, Vy
                    (0x8, _, _, 0x4) => {
                        let sum: u16 = (registers.vx[digit2 as usize] as u16) + (registers.vx[digit3 as usize] as u16);
                        if sum > 255 {
                            registers.vx[0xF] = 1;
                        } else {
                            registers.vx[0xF] = 0;
                        }
                        registers.vx[digit2 as usize] = sum as u8;
                    },
                    // SUB Vx, Vy
                    (0x8, _, _, 0x5) => {
                        registers.vx[0xF] = if registers.vx[digit2 as usize] > registers.vx[digit3 as usize] { 1 } else { 0 };
                        registers.vx[digit2 as usize] = registers.vx[digit2 as usize].wrapping_sub(registers.vx[digit3 as usize]);
                    },
                    // SHR Vx {, Vy}
                    (0x8, _, _, 0x6) => {
                        if registers.vx[digit2 as usize].trailing_ones() > 0 {
                            registers.vx[0xF] = 1;
                        } else {
                            registers.vx[0xF] = 0;
                        }
                        registers.vx[digit2 as usize] = registers.vx[digit2 as usize] >> 1;
                    },
                    // SUBN Vx, Vy
                    (0x8, _, _, 0x7) => {
                        registers.vx[0xF] = if registers.vx[digit3 as usize] > registers.vx[digit2 as usize] { 1 } else { 0 };
                        registers.vx[digit2 as usize] = registers.vx[digit3 as usize].wrapping_sub(registers.vx[digit2 as usize]);
                    },
                    // SHL Vx {, Vy}
                    (0x8, _, _, 0xE) => {
                        if registers.vx[digit2 as usize].leading_ones() > 0 {
                            registers.vx[0xF] = 1;
                        } else {
                            registers.vx[0xF] = 0;
                        }
                        registers.vx[digit2 as usize] = registers.vx[digit2 as usize] << 1;
                    },
                    // SNE, Vx, Vy
                    (0x9, _, _, 0) => {
                        if registers.vx[digit2 as usize] != registers.vx[digit3 as usize] {
                            registers.pc += 2;
                        }
                    },
                    // LD IR, NNN
                    (0xA, _, _, _) => {
                        registers.ir = (digit2 << 8) | (digit3 << 4) | digit4;
                    },
                    // JP, V0 + NNN
                    (0xB, _, _, _) => {
                        registers.pc = (registers.vx[0] as u16) + ((digit2 << 8) | (digit3 << 4) | digit4);
                    },
                    // RND Vx, kk
                    (0xC, _, _, _) => {
                        registers.vx[digit2 as usize] = registers.gen_random() & (((digit3 << 4) | digit4) as u8 );
                    },
                    // DRAW
                    (0xD, _, _, _) => {
                        let x_coord: u8 = registers.vx[digit2 as usize];
                        let y_coord: u8 = registers.vx[digit3 as usize];
                        registers.vx[0xF] = 0;
                        let n_pixels: u16 = digit4;

                        display.update(
                            x_coord,
                            y_coord,
                            n_pixels,
                            &mut registers.vx,
                            registers.ir,
                            &memory.ram,
                        );
                        window.request_redraw();
                    },
                    // SKP Vx
                    (0xE, _, 0x9, 0xE) => {
                        let key = registers.vx[digit2 as usize];
                        if registers.kp[key as usize] {
                            registers.pc += 1;
                        }
                    },
                    // SKNP Vx
                    (0xE, _, 0xA, 0x1) => {
                        let key = registers.vx[digit2 as usize];
                        if !registers.kp[key as usize] {
                            registers.pc += 1;
                        }
                    },
                    // LD, Vx, DT
                    (0xF, _, 0, 0x7) => {
                        registers.vx[digit2 as usize] = registers.dt;
                    },
                    // LD, Vx, K
                    (0xF, _, 0, 0xA) => {
                        let key_pos = registers.kp.iter().position(|&x| x == true);
                        if key_pos.is_some() {
                            registers.vx[digit2 as usize] = key_pos.unwrap() as u8;
                        } else {
                            *control_flow = ControlFlow::Wait;
                        }
                    },
                    // LD DT, Vx
                    (0xF, _, 0x1, 0x5) => {
                        registers.dt = registers.vx[digit2 as usize];
                    },
                    // LD ST, Vx
                    (0xF, _, 0x1, 0x8) => {
                        registers.st = registers.vx[digit2 as usize];
                    },
                    // ADD IR, Vx
                    (0xF, _, 0x1, 0xE) => {
                        registers.ir += registers.vx[digit2 as usize] as u16;
                    },
                    // LD F, Vx
                    (0xF, _, 0x2, 0x9) => {
                        registers.ir = (digit2 as u16) * 5;
                    },
                    // LD B, Vx
                    (0xF, _, 0x3, 0x3) => {
                        let start_addr = registers.ir as usize;
                        let num = registers.vx[digit2 as usize];
                        memory.ram[start_addr + 2] = num % 10;
                        memory.ram[start_addr + 1] = ((num - memory.ram[start_addr + 2]) % 100) / 10;
                        memory.ram[start_addr] = (num - memory.ram[start_addr + 2] - (memory.ram[start_addr + 1] * 10)) / 100;
                    },
                    // LD [I], Vx
                    (0xF, _, 0x5, 0x5) => {
                        let start_addr = registers.ir as usize;
                        memory.ram[start_addr..(start_addr+(digit2 as usize))].copy_from_slice(&registers.vx[0..(digit2 as usize)]);
                    },
                    // LD Vx, [I]
                    (0xF, _, 0x6, 0x5) => {
                        let start_addr = registers.ir as usize;
                        registers.vx[0..(digit2 as usize)].copy_from_slice(&memory.ram[start_addr..(start_addr+(digit2 as usize))]);
                    }
                    _ => {
                        println!("Unknown opcode: {:#x}", opcode);
                    }
                }

                if registers.dt > 0 {
                    registers.dt -= 1;
                }

                if registers.st > 0 {
                    registers.st -= 1;
                }

                registers.kp = vec![false; 16];
            },

            Event::RedrawRequested(_) => {
                display.draw(pixels.frame_mut());
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
        let mut memory = Memory::init();
        memory.load_font();

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
        let mut display = Display::init();

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

    #[test]
    fn test_fx33() {
        let num: u16 = 200;
        let ones = num % 10;
        let tens = ((num - ones) % 100) / 10;
        let hundreds = (num - ones - (tens * 10)) / 100;

        assert_eq!((hundreds, tens, ones), (2, 0, 0));
    }
}
