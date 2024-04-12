# egui_gl_glfw
[![Latest version](https://img.shields.io/crates/v/egui_gl_glfw.svg)](https://crates.io/crates/egui_gl_glfw)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

![Example screenshot](/media/screenshot.png)

[egui_glfw_gl](https://github.com/cohaereo/egui_glfw_gl) is archived so I created this repository and updated
all the dependencies to the latest version as of 2/15/2024. I only made changes to the code where it was
broken from the dependency updates.

This is a backend implementation for [Egui](https://github.com/emilk/egui) that can be used with Rust bindings for [GLFW](https://github.com/PistonDevelopers/glfw-rs) and [OpenGL](https://github.com/brendanzab/gl-rs).

## Example
I have made an example to demonstrate the usage of egui_gl_glfw. To run the example, run the following:
```
cargo run --example demo
```

## Credits
egui_gl_glfw is based on [egui_glfw_gl](https://github.com/cohaereo/egui_glfw_gl) which is based on
[egui_sdl2_gl](https://github.com/ArjunNair/egui_sdl2_gl), created by [ArjunNair](https://github.com/ArjunNair)

## Change Log

### Version 0.1.2

Thanks to [organizedgrime](https://github.com/organizedgrime) for contributing these changes.

* Bump egui to version 0.27
* Expose wayland features from internal crates
