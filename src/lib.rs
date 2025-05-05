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

        let glcore = GLCore::new(|proc_name|window.get_proc_address(proc_name));

        dbg!(glcore);

        while !window.should_close() {
            glfw.poll_events();
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut window, event);
            }
            let cur_frame_time = glfw.get_time();

            glcore.glClearColor(cur_frame_time.sin() as f32, cur_frame_time.cos() as f32, (cur_frame_time * 1.5).sin() as f32, 1.0);
            glcore.glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

            window.swap_buffers();
        }
    }

    #[allow(dead_code)]
    fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
