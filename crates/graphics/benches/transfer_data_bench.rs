use std::hint::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench_group(sample_size = 10000)]
mod spin_mux {

    use spin::Mutex;

    #[divan::bench(threads = 10)]
    fn spin_mux_mutithread10(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock() = vec![1u8; 1024 * 4]);
    }

    #[divan::bench(threads = 2)]
    fn spin_mux_thread2(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock() = vec![1u8; 1024 * 4]);
    }

    #[divan::bench(threads = 1)]
    fn spin_mux_thread1(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock() = vec![1u8; 1024 * 4]);
    }
}

#[divan::bench_group(sample_size = 10000)]
mod mux {
    use std::sync::Mutex;

    #[divan::bench(threads = 10)]
    fn mux_mutithread10(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock().unwrap() = vec![1u8; 1024 * 4]);
    }

    #[divan::bench(threads = 2)]
    fn mux_mutithread2(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock().unwrap() = vec![1u8; 1024 * 4]);
    }

    #[divan::bench(threads = 1)]
    fn mux_mutithread1(bencher: divan::Bencher) {
        let spin_mux = Mutex::new(Vec::new());

        bencher.bench(|| *spin_mux.lock().unwrap() = vec![1u8; 1024 * 4]);
    }
}

#[divan::bench]
fn non_mutx() {
    let mut _spin_mux = false;

    black_box({
        _spin_mux = true;
    })
}

#[divan::bench(sample_size = 10000)]
fn memory_copy(bencher: divan::Bencher) {
    let mut buf = vec![0u8; 1024 * 4];
    let src = vec![1u8; 1024 * 4];

    bencher.bench_local(|| buf.copy_from_slice(&src));
}

#[divan::bench(sample_size = 10000)]
fn mpsc(bencher: divan::Bencher) {
    let src = vec![1u8; 1024 * 4];

    let (sender, receiver) = std::sync::mpsc::channel();

    bencher.bench_local(|| {
        sender.send(src.clone()).unwrap();
        receiver.recv().unwrap();
    });
}
