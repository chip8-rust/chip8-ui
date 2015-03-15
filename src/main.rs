#![cfg_attr(test, allow(dead_code))]

extern crate shader_version;
extern crate input;
extern crate event;
extern crate graphics;
extern crate sdl2_window;
extern crate window;
extern crate opengl_graphics;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rustc_serialize;
extern crate docopt;

extern crate chip8_vm;

use std::cell::RefCell;
use std::rc::Rc;
use sdl2_window::Sdl2Window;
use window::WindowSettings;
use opengl_graphics::{
    Gl,
};

use std::io::Read;
use std::fs::File;
use std::path::Path;
use input::Button;

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

    let (width, height) = (800, 400);
    let opengl = shader_version::OpenGL::_3_2;
    let settings = WindowSettings::new(
        TITLE.to_string(),
        window::Size {
            width: width,
            height: height
        }
    ).exit_on_esc(true);
    let window = Sdl2Window::new(
        opengl,
        settings
    );

    let ref mut gl = Gl::new(opengl);
    let window = Rc::new(RefCell::new(window));

    fn keymap(k: Option<Button>) -> Option<u8> {
        use input::Key::*;
        if let Some(Button::Keyboard(k)) = k {
            return match k {
                D1 => Some(0x1),
                D2 => Some(0x2),
                D3 => Some(0x3),

                Q  => Some(0x4),
                W  => Some(0x5),
                E  => Some(0x6),

                A  => Some(0x7),
                S  => Some(0x8),
                D  => Some(0x9),

                Z  => Some(0xA),
                X  => Some(0x0),
                C  => Some(0xB),

                D4 => Some(0xC),
                R  => Some(0xD),
                F  => Some(0xE),
                V  => Some(0xF),

                _ => None
            }
        }
        return None
    }

    for e in event::events(window.clone()) {
        use event::{ ReleaseEvent, UpdateEvent, PressEvent, RenderEvent };

        if let Some(args) = e.update_args() {
            vm.step(args.dt as f32);
            if vm.beeping() {
                (window.borrow_mut()).window.set_title(BEEP_TITLE).unwrap();
            } else {
                (window.borrow_mut()).window.set_title(TITLE).unwrap();
            }
        }
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
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
