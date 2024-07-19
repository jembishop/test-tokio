use quanta::Clock;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
struct Blob {
    data: Vec<u64>,
}
use tokio;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let clock = Clock::new();
    println!("----SHARED-----");
    shared(&clock).await;
    println!("----COPY-----");
    copied(&clock).await;
}

const ITERS: usize = 64;
const BLOB_SIZE: usize = 1024 * 1024;
const N_TASKS: usize = 1024;

async fn copied(clock: &Clock) {
    let shared_blob = Blob {
        data: vec![1; BLOB_SIZE],
    };
    let mut js = JoinSet::new();
    let start_copy = clock.now();
    let copies: Vec<_> = (0..N_TASKS).map(|_| shared_blob.clone()).collect();
    let after_copy = start_copy.elapsed();
    println!("Took {:?} to copy", after_copy);
    let start_tasks = clock.now();
    for copy in copies {
        js.spawn(async move {
            let mut x = 0;
            for _ in 0..ITERS {
                for i in copy.data.iter() {
                    x += i;
                }
            }
            x
        });
    }
    let mut total_res = 0;
    while let Some(res) = js.join_next().await {
        total_res += res.unwrap();
    }
    let after_tasks = start_tasks.elapsed();
    println!("Took {:?} to run tasks res: {total_res}", after_tasks);
}

async fn shared(clock: &Clock) {
    let shared_blob = Arc::new(RwLock::new(Blob {
        data: vec![1; BLOB_SIZE],
    }));
    let mut js = JoinSet::new();
    let start_copy = clock.now();
    let shared_copies: Vec<_> = (0..N_TASKS).map(|_| shared_blob.clone()).collect();
    let after_copy = start_copy.elapsed();
    println!("Took {:?} to copy", after_copy);
    let start_tasks = clock.now();
    for shared_copy in shared_copies {
        js.spawn(async move {
            let mut x = 0;
            for _ in 0..ITERS {
                for i in shared_copy.read().unwrap().data.iter() {
                    x += i;
                }
            }
            x
        });
    }
    let mut total_res = 0;
    while let Some(res) = js.join_next().await {
        total_res += res.unwrap();
    }
    let after_tasks = start_tasks.elapsed();
    println!("Took {:?} to run tasks res: {total_res}", after_tasks);
}
