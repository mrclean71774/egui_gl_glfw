use {
    egui_backend::{
        egui::{vec2, Color32, Image, Pos2, Rect},
        glfw::Context,
    },
    egui_gl_glfw as egui_backend,
    std::time::Instant,
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PIC_WIDTH: i32 = 320;
const PIC_HEIGHT: i32 = 192;

mod quad;

//Generates a 256 x 256 test texture (RGBA)
fn generate_test_texture_data() -> Vec<u8> {
    let mut data = Vec::with_capacity(256 * 256 * 4);
    for y in 0..256 {
        for x in 0..256 {
            let (r, g, b, a) = if x < 128 && y < 128 || x >= 128 && y >= 128 {
                (255, 0, 255, 255) //Purple
            } else {
                (0, 0, 0, 255) //Black
            };
            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
    }
    data
}

fn output_gl_errors() {
    unsafe {
        let mut err = gl::GetError();
        while err != gl::NO_ERROR {
            eprintln!("OpenGL Error: {err}");
            err = gl::GetError();
        }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Egui in GLFW!",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut painter = egui_backend::Painter::new(&mut window);
    let egui_ctx = egui::Context::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            ..Default::default()
        },
        native_pixels_per_point,
    );

    let start_time = Instant::now();
    let srgba = vec![Color32::BLACK; (PIC_HEIGHT * PIC_WIDTH) as usize];

    let plot_tex_id = painter.new_user_texture(
        (PIC_WIDTH as usize, PIC_HEIGHT as usize),
        &srgba,
        egui::TextureFilter::Linear,
    );

    let mut sine_shift = 0f32;
    let mut amplitude = 50f32;
    let mut test_str =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let quad = quad::Quad::new();
    let mut quit = false;

    //Generate test texture
    let mut texture_id = 0;
    unsafe {
        let texture_data = generate_test_texture_data();
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TextureParameteri(texture_id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TextureParameteri(texture_id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            256,
            256,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture_data.as_ptr() as *const std::os::raw::c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    while !window.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_pass(egui_input_state.input.take());
        egui_input_state.pixels_per_point = native_pixels_per_point;

        unsafe {
            gl::ClearColor(0.455, 0.302, 0.663, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
        quad.draw();

        let mut srgba: Vec<Color32> = Vec::new();
        let mut angle = 0f32;

        for y in 0..PIC_HEIGHT {
            for x in 0..PIC_WIDTH {
                srgba.push(Color32::BLACK);
                if y == PIC_HEIGHT - 1 {
                    let y = amplitude * (angle * std::f32::consts::PI / 180f32 + sine_shift).sin();
                    let y = PIC_HEIGHT as f32 / 2f32 - y;
                    srgba[(y as i32 * PIC_WIDTH + x) as usize] = Color32::YELLOW;
                    angle += 360f32 / PIC_WIDTH as f32;
                }
            }
        }
        sine_shift += 0.1f32;

        //This updates the previously initialized texture with new data.
        //If we weren't updating the texture, this call wouldn't be required.
        painter.update_user_texture_data(&plot_tex_id, &srgba);

        egui::Window::new("Egui with GLFW").show(&egui_ctx, |ui| {
            egui::TopBottomPanel::top("Top").show(&egui_ctx, |ui| {
                ui.menu_button("File", |ui| {
                    {
                        let _ = ui.button("test 1");
                    }
                    ui.separator();
                    {
                        let _ = ui.button("test 2");
                    }
                });
            });

            //Image just needs a texture id reference, so we just pass it the texture id that was returned to us
            //when we previously initialized the texture.
            ui.add(Image::new(egui::load::SizedTexture{id:plot_tex_id, size: vec2(PIC_WIDTH as f32, PIC_HEIGHT as f32)}));
            ui.separator();
            ui.label("A simple sine wave plotted onto a GL texture then blitted to an egui managed Image.");
            ui.label(" ");
            ui.text_edit_multiline(&mut test_str);
            ui.label(" ");            
            ui.add(egui::Slider::new(&mut amplitude, 0.0..=50.0).text("Amplitude"));
            ui.label(" ");
            if ui.button("Quit").clicked() {
                quit = true;
            }
        });

        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.end_pass();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.

        let clipped_shapes = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
            }
        }
        window.swap_buffers();
        glfw.poll_events();
        output_gl_errors();

        if quit {
            break;
        }
    }
}
