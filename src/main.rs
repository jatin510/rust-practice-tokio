use std::sync::Arc;
use std::thread;
use tokio::runtime::Builder;
use tokio::task;
use tokio::time::{Duration, sleep};

fn main() {
    let mut counter = Arc::new(0);
    // EmailJob
    let email_handle = thread::spawn(move || {
        // some work here
        let runtime = Builder::new_multi_thread()
            .worker_threads(1)
            .thread_name("email-thread")
            .enable_time()
            .build()
            .unwrap();

        runtime.block_on(async move { email_job(counter).await });
    });

    // Data processing
    let data_handle = thread::spawn(move || {
        // some work here
        let runtime = Builder::new_multi_thread()
            .worker_threads(9)
            .thread_name("data-thread")
            .enable_time()
            .build()
            .unwrap();

        runtime.block_on(async { data_processing_job().await });
    });

    email_handle.join().unwrap();
    data_handle.join().unwrap();
}

async fn email_job(counter: Arc<i32>) {
    let total_worker_thread = 5;

    for _ in 0..total_worker_thread {
        task::spawn(async {
            let email_job = EmailJob::new(
                //  shared state and mpsc
            );
            email_job.run().await;
        });
    }

    // suppose this is a mpsc channel which is receiving data
    loop {}
}

async fn data_processing_job() {
    let total_worker_thread = 5;

    for _ in 0..total_worker_thread {
        task::spawn(async {
            let data_processing_job = DataProcessingJob::new(
                //  shared state and mpsc
            );
            data_processing_job.run().await;
        });
    }

    // suppose this is a mpsc channel which is receiving data
    loop {}
}

struct EmailJob {}

impl EmailJob {
    fn new() -> Self {
        EmailJob {}
    }

    async fn run(&self) {
        println!("email job running");
        sleep(Duration::from_secs(1)).await
    }
}

struct DataProcessingJob {}

impl DataProcessingJob {
    fn new() -> Self {
        DataProcessingJob {}
    }

    async fn run(&self) {
        println!("worker job running");
        sleep(Duration::from_secs(5)).await
    }
}
