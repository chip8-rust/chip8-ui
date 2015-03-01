Chip-8 emulator in Rust
==
This is an emulator with a GUI Built on top of the `chip8_vm` library.

See [chip8_vm](https://github.com/chip8-rust/chip8-vm) for instructions on including the VM in your own project.

![screen shot - pong2](https://cloud.githubusercontent.com/assets/322861/6091672/b7db0636-aefa-11e4-84f3-24d66e06dbba.png)
*Running [PONG2](http://www.chip8.com/?page=109) from chip8.com program pack*

Status
==
[![Build Status](https://travis-ci.org/chip8-rust/chip8-ui.svg?branch=master)](https://travis-ci.org/chip8-rust/chip8-ui)
* Graphics are implemented with [Piston](http://www.piston.rs/).
* Sound is not supported but is faked by updating the title bar with a note
symbol when sound should be playing.

Dependencies
==
The provided UI depends on SDL2 via [Piston](http://www.piston.rs/) for it's graphics. See: [Piston Tutorials - Getting Started](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started) for help setting up your development environment.

Usage
==

```sh
$ cargo run --release -- <rom>
```

Controls are mapped to these 16 buttons:

  1  |  2  |  3  |  4
-----|-----|-----|-----
  Q  |  W  |  E  |  R
  A  |  S  |  D  |  F
  Z  |  X  |  C  |  V

Licence
==
MIT
