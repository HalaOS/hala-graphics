use std::hint::black_box;

use divan::Bencher;
use futures::executor::block_on;
use hala_graphics::{render::Compositor, Viewport};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn compositing(bencher: Bencher) {
    let mut compositor =
        block_on(Compositor::new().render_to_texture(Viewport::new(256, 256))).unwrap();

    bencher.bench_local(|| black_box(compositor.compositing().unwrap()));
}
