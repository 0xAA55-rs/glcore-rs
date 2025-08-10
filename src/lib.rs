mod glcore;

pub use glcore::*;

#[cfg(test)]
mod tests {
    use glfw::{Action, Context, Key, SwapInterval};
    use crate::glcore::*;

    #[test]
    fn test() {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();
        glfw.set_swap_interval(SwapInterval::Adaptive);

        let glcore = GLCore::new(|proc_name|window.get_proc_address(proc_name)).unwrap();
        dbg!(glcore);

        let start_time = glfw.get_time();
        while !window.should_close() {
            let cur_frame_time = glfw.get_time();

            glcore.glClearColor(cur_frame_time.sin() as f32, cur_frame_time.cos() as f32, (cur_frame_time * 1.5).sin() as f32, 1.0).unwrap();
            glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT).unwrap();

            window.swap_buffers();
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    _ => {}
                }
            }
            if cur_frame_time - start_time >= 10.0 {
                window.set_should_close(true)
            }
        }
    }
}
