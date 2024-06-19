use hala_canvas_wgpu::test::TestRunner;

fn main() {
    pretty_env_logger::init_timed();
    TestRunner::run().unwrap();
}
