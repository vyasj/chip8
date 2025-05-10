use muda::{Menu, MenuEvent, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{ElementState, Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::platform::unix::WindowExtUnix as _;
use tao::window::WindowBuilder;

use chip8::cpu::{Cpu,Instruction};

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

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
                }

                WindowEvent::Resized(size) => {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("{}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }

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
                }

                _ => {
                    //
                }
            },

            Event::MainEventsCleared => {
                if cpu.pc as usize >= cpu.ram.len() {
                    println!("reached end of ram.");
                    cpu.dump_state();
                    std::process::exit(0x0100);
                }

                if cpu.ram[cpu.pc as usize] == 0x00 && cpu.ram[(cpu.pc + 1) as usize] == 0x00 {
                    println!("hit raw 0x0000");
                    cpu.dump_state();
                    std::process::exit(0x0100);
                }

                let bytes = cpu.fetch();
                let instruction = Instruction::decode(bytes).unwrap();
                // Instruction::print_name(&instruction);
                let result = cpu.execute(instruction);

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

                if cpu.dt > 0 {
                    cpu.dt -= 1;
                }

                if cpu.st > 0 {
                    cpu.st -= 1;
                }

                cpu.kp = vec![false; 16];
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
    use chip8::cpu::Instruction;

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

    #[test]
    fn test_00e0() {
        // CLS
        let mut cpu = Cpu::init();
        cpu.screen = vec![true; 64 * 32];

        cpu.ram[0x200] = 0x00;
        cpu.ram[0x201] = 0xe0;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.screen, vec![false; 64 * 32]);
    }

    #[test]
    fn test_00ee() {
        // RET
        for stack_idx in 1..=15 {
            let mut cpu = Cpu::init();
            let ret_addr: u16 = 0x250;
            cpu.sp = stack_idx;
            cpu.stack[cpu.sp as usize] = ret_addr;

            cpu.ram[0x200] = 0x00;
            cpu.ram[0x201] = 0xee;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.execute(ins);

            assert_eq!(cpu.pc, ret_addr);
            assert_eq!(cpu.sp, stack_idx - 1);
        }
    }

    #[test]
    fn test_1nnn() {
        // JP addr
        let mut cpu = Cpu::init();
        let jp_addr: u16 = 0x250;

        let tmp = 0x1000 | jp_addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let opcode = cpu.fetch();
        let ins = Instruction::decode(opcode).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, jp_addr);
    }

    #[test]
    fn test_2nnn() {
        // CALL addr
        for stack_idx in 0..=14 {
            let mut cpu = Cpu::init();
            let sr_addr: u16 = 0x250;
            cpu.sp = stack_idx;

            let tmp = 0x2000 | sr_addr;
            cpu.ram[0x200] = (tmp >> 8) as u8;
            cpu.ram[0x201] = tmp as u8;

            let opcode = cpu.fetch();
            let ins = Instruction::decode(opcode).unwrap();
            let _ = cpu.execute(ins);

            assert_eq!(cpu.stack[cpu.sp as usize], 0x202);
            assert_eq!(cpu.pc, 0x250);
            assert_eq!(cpu.sp, stack_idx + 1);
        }
    }

    #[test]
    fn test_3xkk() {
        // SE x, kk
        let mut cpu = Cpu::init();
        let kk: u8 = 0x25;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = kk;

        let tmp: u16 = 0x3000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_4xkk() {
        // SNE x, kk
        let mut cpu = Cpu::init();
        let kk: u8 = 0x24;
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0x3000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn test_5xy0() {
        // SE x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let val: u8 = 0x25;
        cpu.vx[x as usize] = val;
        cpu.vx[y as usize] = val;

        let tmp: u16 = 0x5000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_6xkk() {
        // LD x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;

        let tmp: u16 = 0x6000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], kk);
    }

    #[test]
    fn test_7xkk() {
        // ADD x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let kk: u8 = 0x25;
        let orig: u8 = 0x40;
        cpu.vx[x as usize] = orig;

        let tmp = 0x7000 | ((x as u16) << 8) | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], orig + kk);
    }

    #[test]
    fn test_8xy0() {
        // LD x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        cpu.vx[y as usize] = 0x25;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.vx[y as usize]);
    }

    #[test]
    fn test_8xy1() {
        // OR x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x1;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val | y_val);
    }

    #[test]
    fn test_8xy2() {
        // AND x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x2;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val & y_val);
    }

    #[test]
    fn test_8xy3() {
        // XOR x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x25;
        let y_val: u8 = 0x80;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x3;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val ^ y_val);
    }

    #[test]
    fn test_8xy4() {
        // ADD x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x01;
        let y_val: u8 = 0xFF;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let sum: u16 = (x_val as u16) + (y_val as u16);

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x4;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], sum as u8);
        assert_eq!(cpu.vx[0xF] == 1, (0xFF00 & sum) > 1);
    }

    #[test]
    fn test_8xy5() {
        // SUB x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x01;
        let y_val: u8 = 0xFF;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let diff: u16 = if x_val > y_val {
            (x_val as u16) - (y_val as u16)
        } else {
            (y_val as u16) - (x_val as u16)
        };

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x5;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, x_val > y_val);
    }

    #[test]
    fn test_8xy6() {
        // SHR x {, y}
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x6;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val >> 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.trailing_ones() > 1);
    }

    #[test]
    fn test_8xy7() {
        // SUBN x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0xFF;
        let y_val: u8 = 0x01;
        cpu.vx[x as usize] = x_val;
        cpu.vx[y as usize] = y_val;
        let diff: u16 = if y_val > x_val {
            (y_val as u16) - (x_val as u16)
        } else {
            (x_val as u16) - (y_val as u16)
        };

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0x7;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], diff as u8);
        assert_eq!(cpu.vx[0xF] == 1, y_val > x_val);
    }

    #[test]
    fn test_8xye() {
        // SHL x {, y}
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let x_val: u8 = 0x4;
        cpu.vx[x as usize] = x_val;

        let tmp = 0x8000 | ((x as u16) << 8) | ((y as u16) << 4) | 0xE;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], x_val << 1);
        assert_eq!(cpu.vx[0xF] == 1, x_val.leading_ones() > 1);
    }

    #[test]
    fn test_9xy0() {
        // SNE x, y
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        let val: u8 = 0x25;
        cpu.vx[x as usize] = val;
        cpu.vx[y as usize] = val + 1;

        let tmp: u16 = 0x9000 | ((x as u16) << 8) | ((y as u16) << 4);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_annn() {
        // LD I, addr
        let mut cpu = Cpu::init();
        let addr: u16 = 0x250;

        let tmp: u16 = 0xA000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, addr);
    }

    #[test]
    fn test_bnnn() {
        // JP V0, addr
        let mut cpu = Cpu::init();
        cpu.vx[0x0] = 0x25;
        let addr: u16 = 0x250;

        let tmp: u16 = 0xB000 | addr;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, (cpu.vx[0x0] as u16) + addr);
    }

    #[test]
    fn test_cxkk() {
        // RND x, kk
        let mut cpu = Cpu::init();
        let x: u8 = 0xF;
        let kk: u8 = 0xFF;
        let val: u8 = 0xF;
        cpu.vx[x as usize] = val;

        let tmp: u16 = 0xC000 | (x as u16) << 8 | (kk as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], 0);
    }

    #[test]
    fn test_dxyn() {
        // DRW x, y, n
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let y: u8 = 0xA;
        cpu.vx[x as usize] = 0x5;
        cpu.vx[y as usize] = 0x7;
        cpu.ir = 0x250;
        let n: u8 = 2;

        let tmp: u16 = 0xD000 | ((x as u16) << 8) | ((y as u16) << 4) | (n as u16);
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;
        for i in 0..n {
            cpu.ram[(cpu.ir + i as u16) as usize] = i;
        }
        cpu.screen[0x20C] = true;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[0xF], 1);
        assert_eq!(cpu.screen[0x20C], false);
    }

    #[test]
    fn test_ex9e() {
        // SKP x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0x9E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_exa1() {
        // SKNP x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.vx[x as usize] = key;
        cpu.kp[(key - 1) as usize] = true;

        let tmp: u16 = 0xE000 | ((x as u16) << 8) | 0xA1;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn test_fx07() {
        // LD x, dt
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.dt = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x7;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx0a() {
        // LD x, kp
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let key: u8 = 0x5;
        cpu.kp[key as usize] = true;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0xA;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], key);
    }

    #[test]
    fn test_fx15() {
        // LD dt, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.dt = 0x25;
        
        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x15;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.dt);
    }

    #[test]
    fn test_fx18() {
        // LD st, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.st = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x18;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.vx[x as usize], cpu.st);
    }

    #[test]
    fn test_fx1e() {
        // ADD ir, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        let initial_ir: u16 = 0xF01E;
        cpu.ir = initial_ir;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x1E;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) + initial_ir);
    }

    #[test]
    fn test_fx29() {
        // LD F, vx
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 0x25;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x29;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ir, (cpu.vx[x as usize] as u16) * 5);
    }

    #[test]
    fn test_fx33() {
        // LD B, x
        let mut cpu = Cpu::init();
        let x: u8 = 0x1;
        cpu.vx[x as usize] = 137;
        cpu.ir = 0x250;

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x33;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ram[cpu.ir as usize], 1);
        assert_eq!(cpu.ram[(cpu.ir+1) as usize], 3);
        assert_eq!(cpu.ram[(cpu.ir+2) as usize], 7);
    }

    #[test]
    fn test_fx55() {
        // LD [ir], x
        let mut cpu = Cpu::init();
        let x: u8 = 0x5;
        for i in 0..x {
            cpu.vx[i as usize] = i * 16;
        }

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x55;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)], cpu.vx[0..=(x as usize)]);
    }

    #[test]
    fn test_fx65() {
        // LD x, [ir]
        let mut cpu = Cpu::init();
        let x: u8 = 0x5;
        cpu.ir = 0x250;
        for i in 0..x {
            cpu.ram[(cpu.ir as usize) + (i as usize)] = i * 16;
        }

        let tmp: u16 = 0xF000 | ((x as u16) << 8) | 0x65;
        cpu.ram[0x200] = (tmp >> 8) as u8;
        cpu.ram[0x201] = tmp as u8;

        let instruction = cpu.fetch();
        let ins = Instruction::decode(instruction).unwrap();
        let _ = cpu.execute(ins);

        assert_eq!(cpu.ram[(cpu.ir as usize)..=((cpu.ir + (x as u16)) as usize)], cpu.vx[0..=(x as usize)]);
    }
}