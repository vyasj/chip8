CHIP-8 Emulator that I was recommended to make before attempting to make a Gameboy Advance emulator (https://github.com/vyasj/rustboy, check it out #ad)

In Rust, because I hate myself! :D

## Usage

Ensure that the ROM you want to run is in the `roms/` directory, either under `games/` or `tests/`. Then, simply run `cargo run path/to/game`. For example, `cargo run games/Tetris.ch8`.

## References
- https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
- https://github.com/corax89/chip8-test-rom
- https://github.com/kripod/chip8-roms
- https://github.com/parasyte/pixels/blob/main/examples/minimal-tao/src/main.rs