use concurrency::AmapMetrics;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() {
    let metrics = AmapMetrics::new();

    println!("Metrics snapshot: {:?}", metrics.snapshot());

    // 模拟 metrics
    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }

    // 模拟 metrics
    for _ in 0..M {
        request_worker(metrics.clone());
    }

    // Print metrics snapshot every 2 seconds.
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Metrics snapshot: {:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: AmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", idx)).unwrap();
    });
}

fn request_worker(metrics: AmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page)).unwrap();
    });
}
