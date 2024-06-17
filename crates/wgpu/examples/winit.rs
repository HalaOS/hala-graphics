use hala_canvas_wgpu::Application;

fn main() {
    pretty_env_logger::init_timed();
    Application::run().unwrap();
}
