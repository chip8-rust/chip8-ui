#![cfg_attr(test, allow(dead_code))]

extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate sdl2_window;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rustc_serialize;
extern crate docopt;

extern crate chip8_vm;

use opengl_graphics::{ GlGraphics, OpenGL };
use std::rc::Rc;
use std::cell::RefCell;
use piston::window::{ AdvancedWindow, WindowSettings };
use piston::input::{ Button, Key };
use piston::event::*;
use sdl2_window::Sdl2Window as Window;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;

use chip8_vm::vm::Vm;

const TITLE: &'static str = "Chip8";
const BEEP_TITLE: &'static str = "♬ Chip8 ♬";

const USAGE: &'static str = "
CHIP-8 user interface.

Usage:
  chip8_ui [--] <rom> | -
  chip8_ui -h | --help
  chip8_ui -V | --version

Options:
  -h, --help     Show this screen.
  -V, --version  Show version.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_rom: String,
}

fn create_vm<R>(rom: &mut R) -> Vm
    where R: Read
{
    let mut vm = Vm::new();
    match vm.load_rom(rom) {
        Ok(_) => vm,
        Err(e) => panic!("Error loading ROM: {}", e)
    }
}

fn version() -> &'static str {
    // TODO: There's also an optional _PRE part
    concat!(
        env!("CARGO_PKG_VERSION_MAJOR"), ".",
        env!("CARGO_PKG_VERSION_MINOR"), ".",
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}

fn main() {
    env_logger::init().unwrap();

    let docopt = Docopt::new(USAGE).unwrap()
                        .help(true)
                        .version(Some(version().to_string()));
    let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());
    debug!("CLI args: {:?}", args);

    let mut vm = if "-" == args.arg_rom {
        create_vm(&mut std::io::stdin())
    }
    else {
        let mut file = File::open(&Path::new(&args.arg_rom)).unwrap();
        create_vm(&mut file)
    };

    let opengl = OpenGL::_3_2;

    let window: Window = WindowSettings::new(
        TITLE.to_string(),
        [800, 400]
    )
    .exit_on_esc(true)
    .opengl(opengl)
    .into();

    let ref mut gl = GlGraphics::new(opengl);
    let window = Rc::new(RefCell::new(window));

    fn keymap(k: Option<Button>) -> Option<u8> {
        if let Some(Button::Keyboard(k)) = k {
            return match k {
                Key::D1 => Some(0x1),
                Key::D2 => Some(0x2),
                Key::D3 => Some(0x3),

                Key::Q  => Some(0x4),
                Key::W  => Some(0x5),
                Key::E  => Some(0x6),

                Key::A  => Some(0x7),
                Key::S  => Some(0x8),
                Key::D  => Some(0x9),

                Key::Z  => Some(0xA),
                Key::X  => Some(0x0),
                Key::C  => Some(0xB),

                Key::D4 => Some(0xC),
                Key::R  => Some(0xD),
                Key::F  => Some(0xE),
                Key::V  => Some(0xF),

                _ => None
            }
        }
        return None
    }

    for e in window.clone().events() {
        if let Some(args) = e.update_args() {
            vm.step(args.dt as f32);
            if vm.beeping() {
                window.borrow_mut().set_title(BEEP_TITLE.to_string());
            } else {
                window.borrow_mut().set_title(TITLE.to_string());
            }
        }
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
                let r = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
                let off = [0.0, 0.0, 0.0, 1.0];
                let on = [1.0, 1.0, 1.0, 1.0];

                let w = args.width as f64 / 64.0;
                let h = args.height as f64 / 32.0;

                for (y,row) in vm.screen_rows().enumerate() {
                    for (x,byte) in row.iter().enumerate() {
                        let x = x as f64 * w;
                        let y = y as f64 * h;
                        let color = match *byte { 0 => off, _ => on };
                        r.color(color).draw([x, y, w, h], &c.draw_state, c.transform, gl);
                    }
                }
            });
        }
        if let Some(keynum) = keymap(e.press_args()) {
            vm.set_key(keynum);
        }
        if let Some(keynum) = keymap(e.release_args()) {
            vm.unset_key(keynum);
        }
    }
}
