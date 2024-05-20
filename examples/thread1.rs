use anyhow::{anyhow, Result};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    val: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // close the channel

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }

        "Consumer exit".to_string()
    });

    let consumer_result = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    println!("Consumer result: {:?}", consumer_result);

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;

        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // exit the loop after random time
        if rand::random::<u8>() % 5 == 0 {
            println!("Producer {} exiting", idx);
            break;
        }
    }

    Ok(())
}

impl Msg {
    fn new(idx: usize, val: usize) -> Msg {
        Msg { idx, val }
    }
}
