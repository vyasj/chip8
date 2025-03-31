use muda::{Menu, MenuEvent, Submenu};
use pixels::{Error, Pixels, SurfaceTexture};
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::WindowBuilder;
use tao::platform::unix::WindowExtUnix as _;

use chip8::mem::Memory;
use chip8::disp::Display;
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
            .with_title("CHIP-8 bullshit")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap();
        Arc::new(window)
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, Arc::clone(&window));
        Pixels::new(display.width as u32, display.height as u32, surface_texture)?
    };

    let menu = Menu::new();

    use tao::platform::unix::WindowExtUnix as _;
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

                WindowEvent::Resized(size) => {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        println!("{}", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {}
            },

            Event::MainEventsCleared => {
                let n1: u8 = memory.ram[registers.pc as usize];
                registers.pc = registers.pc + 1;
                let n2: u8 = memory.ram[registers.pc as usize];
                registers.pc = registers.pc + 1;

                let opcode: u16 = ((n1 as u16) << 8) | n2 as u16;
                let nibble1: u16 = 0xF000 & opcode;
                let nibblex: u16 = 0x0F00 & opcode;
                let nibbley: u16 = 0x00F0 & opcode;
                let nibbleN: u16 = 0x000F & opcode;
                let nibbleNN: u16 = 0x00FF & opcode;
                let nibbleNNN: u16 = 0x0FFF & opcode;

                if nibble1 == 0x0000 {
                    println!("special subroutine call, very serious stuff. kill yourself.");
                }

                if opcode == 0x00E0 {
                    // clear screen
                    for x in &mut display.screen {
                        *x = 0;
                    }
                } else if nibble1 == 0x1000 {
                    // jump
                    registers.pc = nibbleNNN;
                } else if nibble1 == 0x6000 {
                    // set register vx
                    let reg_num = (nibblex >> 8) as usize;
                    let val = nibbleNN as u8;
                    registers.vx[reg_num] = val;
                } else if nibble1 == 0x7000 {
                    // add value to register vx
                    let reg_num = (nibblex >> 8) as usize;
                    let val = nibbleNN as u8;
                    registers.vx[reg_num] = registers.vx[reg_num] + val;
                } else if nibble1 == 0xA000 {
                    // set index register
                    registers.ir = nibbleNNN;
                } else if nibble1 == 0xD000 {
                    // draw
                    let x_coord = registers.vx[(nibblex >> 8) as usize] & 63;
                    let y_coord = registers.vx[(nibbley >> 8) as usize] & 32;
                    registers.vx[0x0F] = 0;
                    let n_pixels = nibbleN;

                    display.update();
                    window.request_redraw();
                }
            }

            Event::RedrawRequested(_) => {
                display.draw(pixels.frame_mut());
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

    Ok(())
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        assert_eq!(memory.ram[0x050..0x0A0], exp_result);
    }

    #[test]
    fn test_rom1() {
        let filename = "test_opcode.ch8";

        let mut memory = Memory::init();
        memory.load_font();

        memory.load_rom(filename);

        assert!(memory.ram[0x200] != 0);
    }

    #[test]
    fn test_display() {
        let mut display = Display::init();

        let screen_copy: Vec<u8> = vec![0; 2048];

        display.screen[0..32].fill(1);

        let mut exp_result: Vec<u8> = vec![0; 2048];
        exp_result[0..32].fill(1);

        let result: Vec<u8> = display.screen
            .iter()
            .zip(screen_copy.iter())
            .map(|(&x1, &x2)| x1 ^ x2)
            .collect();

        assert_eq!(exp_result, result);
    }

    #[test]
    fn test_bit_shit() {
        let num: u16 = 0x70FB;

        let exp_result: Vec<u8> = vec![0x70, 0xFB];

        // testing to_be_bytes()
        // let result = num.to_be_bytes();

        // testing as keyword
        let first: u8 = (num >> 8) as u8;
        let second: u8 = num as u8;
        let result = vec![first, second];
        
        assert_eq!(exp_result, result);
    }
}