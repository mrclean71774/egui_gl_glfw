# egui_gl_glfw

[![Latest version](https://img.shields.io/crates/v/egui_gl_glfw.svg)](https://crates.io/crates/egui_gl_glfw)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

![Example screenshot](/media/screenshot.png)

[egui_glfw_gl](https://github.com/cohaereo/egui_glfw_gl) is archived so I created this repository and made changes to the code where it was broken from the dependency updates.

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

### Version 0.1.3

* Bump egui to 0.27.2
* Bump glfw to 0.57.0
* Some feature changes due to glfw changes

### Version 0.1.4

* Bump egui to 0.28.1
* Bump glfw to 0.58.0

### Version 0.1.5

* Add features to control most egui and glfw features

### Version 0.1.6

* Improved mouse button handling, Thanks to [JLi69](https://github.com/JLi69)

### Version 0.1.7

* Bump glfw to 0.59.0
