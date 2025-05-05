# The OpenGL core functions for Rust

## Usage:
1. You need `wglGetProcAddress()` on Windows or `glxGetProcAddress` on Linux or something else, to get the function address of the OpenGL functions your system provides.
   For example, the code below uses `GLFW` library, it provides `window.get_proc_address()` could do this.
2. Create the `GLCore` object by this:
   `let glcore = GLCore::new(|proc_name|your_get_proc_address(proc_name));`
3. BAM. All of the OpenGL core functions are here callable through the `glcore`
4. This also supports OpenGL ES, Mesa, or others. Basically something like GLEW, but in pure Rust.

## Look at the example of using OpenGL with GLFW:

```rust
use glfw::{Action, Context, Key, SwapInterval};
use glcore::*;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(640, 480, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(SwapInterval::Adaptive);

    // Look at here, this is the OpenGL core function lib creation
    let glcore = GLCore::new(|proc_name|window.get_proc_address(proc_name));

    dbg!(glcore);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        let cur_frame_time = glfw.get_time();

        // Call the OpenGL functions here.
        glcore.glClearColor(cur_frame_time.sin() as f32, cur_frame_time.cos() as f32, (cur_frame_time * 1.5).sin() as f32, 1.0);
        glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

        // Show the rendered content
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
```
