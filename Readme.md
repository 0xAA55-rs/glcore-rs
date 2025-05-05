# The OpenGL core functions for Rust

* The main `struct` and `trait` for you to use are `GLCore` and `GL`.
  The struct `GLCore` stores all function pointers, and `GL` has all OpenGL core function prototype definitions.
  `GLCore` implements all of the OpenGL core functions that the trait `GL` declared.
* `GLCore` derives `Debug`, `Clone`, `Copy`, `PartialEq`, and `Eq`, is very handy to use, you can copy it everywhere, it's just a bunch of function pointers.
* When created, every function pointer would not be NULL.
  If some OpenGL functions are not retrievable by calling `get_proc_address()`, for the sake of safety, the function pointer will be replaced by a dummy function that calls `panic!()` for you to help diagnose which parts of your code are wrong.
* OpenGL/ES contexts created from `libEGL` are also usable by `GLCore`. Use `eglGetProcAddress()` to create `GLCore`.
* `GLCore` also supports OpenGL ES, with some of the OpenGL functions which don't belong to OpenGL ES not able to be used even if they look like exist in the trait or the struct. Read the specifications of OpenGL ES carefully, and use the supported functions that were supported by OpenGL ES. Using the unsupported functions causes `panic!()` for you to help diagnose which parts of your code are wrong.

## Usage:
1. You need a created OpenGL context. The easiest way is to use the `GLFW` library that creates a window with an OpenGL context for you. `GLFW` is crossplatform, so do the `glcore`.
2. You need `wglGetProcAddress()` on Windows or `glxGetProcAddress()` on Linux with an X Server that supports GLX, or on the other platform that supports EGL, you have `eglGetProcAddress()`. Or something else, to get the function address of the OpenGL functions your system provides.
   For example, the code below uses `GLFW` library, it provides `window.get_proc_address()` could do this.
3. Create the `GLCore` object by this:
   `let glcore = GLCore::new(|proc_name|your_get_proc_address(proc_name));`
4. BAM. All of the OpenGL core functions are here callable through the `glcore`

## Multiple card or multiple thread usage
* If you have more than one graphic card regardless of their vendors are same (e.g. One is NVIDIA, and another is AMD),  you can create different OpenGL contexts and environments, or even get one more  `get_proc_address()` function from your different card vendor APIs.
* You can then create one `GLCore` in a specific OpenGL context that contains functions from NVIDIA, and another `GLCore` from another specific OpenGL context that contains functions from AMD graphic driver, use them at the same time, and let both of the cards compute for you. You can even create some threads dedicated to each OpenGL context, call `make_current()` on each thread, and use your cards concurrently.

## OpenGL tips

### For scene rendering usage

* OpenGL 3.3 is enough for complicated scene rendering, and that's also the most commonly supported version even in some embedded systems.
* By using OpenGL 3.x for scene rendering, you have VBO to manage your meshes, you have shader programs for GPU rendering, you have VAO to link your mesh attrib to a shader program, you have batch rendering command `glDrawXxxxInstanced()`, `glMultiDrawXxxxxBaseVertex()` etc.
* The most powerful batch rendering command is `glMultiDrawElementsIndirect` but that needs OpenGL 4.3.
* Do not use OpenGL 2.x and 1.x ways to render things, there are too many limitations of using shaders, and if you are not using VBO, drawing any primitive shapes is slow because the shapes or meshes are not stored in the GPU GRAM, every time you want to draw, the CPU send the mesh data to the GPU, that's why it's slow.

### For GPU computing usage

* You must have OpenGL 4.3 context, which provides `glDispatchCompute()` for GPU computing, it uses shaders, images, and groups jobs in 3 dimensions, almost like using CUDA.
* After called this function, the GPU starts computing, and your CPU are not just waiting for it to complete.
* When you have to retrieve the computed result, call `glMemoryBarrier(GL_SHADER_IMAGE_ACCESS_BARRIER_BIT)`, then you can access the images to retrieve it back to your RAM.

## Example code

### Look at the example of using OpenGL with GLFW:

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

        // Because the `GLCore` is the almost naked OpenGL functions provider, I don't want to write some complicated scene rendering codes here.
        // Just changes the background color that indicates the OpenGL functions were working.

        // Let the GLFW window shows the rendered content for you
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

## NOTE

* Make sure only call functions that were provided by your system.
  e.g. If your system provides an OpenGL version is 4.0.0, do not try to call 4.1.0 functions.
  See the source code of `GLCore` you will find out which function was introduced from which version of OpenGL.
* Create the `GLCore` inside the OpenGL context (which mean after you called your `make_current()`). The OpenGL context and the `get_proc_address()` behave differently in different graphic card drivers.
